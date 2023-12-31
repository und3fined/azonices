// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use sanakirja::{btree, Env, LoadPage, RootDb, RootPage};

use crate::{
  models::{compartment::SerializedCompartment, entry::SerializedEntry, filter::SerializedFilter, label::SerializedLabel, space, vault::SerializedVault},
  traits::{MutTxnT, TxnT},
  types::*,
};

use super::{sanakirja::types::*, Encyc, EncycError};

pub struct ArcTxn<T>(pub Arc<RwLock<T>>);

impl<T> ArcTxn<T> {
  pub fn new(t: T) -> Self {
    ArcTxn(Arc::new(RwLock::new(t)))
  }
}

impl<T> Clone for ArcTxn<T> {
  fn clone(&self) -> Self {
    ArcTxn(self.0.clone())
  }
}

impl<T: MutTxnT> ArcTxn<T> {
  pub fn commit(self) -> Result<(), T::GraphError> {
    if let Ok(txn) = Arc::try_unwrap(self.0) {
      txn.into_inner().commit()
    } else {
      panic!("Tried to commit an ArcTxn without dropping its references")
    }
  }
}

impl<T> std::ops::Deref for ArcTxn<T> {
  type Target = RwLock<T>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
pub enum Root {
  Version,
  Entries,
  Compartments,
  Labels,
  Filters,
  Vaults,
  Spaces,
}

pub const VERSION: L64 = L64(1u64.to_le());

impl Encyc {
  pub fn txn_begin(&self) -> Result<Txn, EncycError> {
    let txn = Env::txn_begin(self.env.clone())?;
    if L64(txn.root(Root::Version as usize)) != VERSION {
      return Err(EncycError::Version);
    }

    fn begin(txn: sanakirja::Txn<Arc<Env>>) -> Option<Txn> {
      Some(Txn {
        entries: txn.root_db(Root::Entries as usize)?,
        compartments: txn.root_db(Root::Compartments as usize)?,
        labels: txn.root_db(Root::Labels as usize)?,
        filters: txn.root_db(Root::Filters as usize)?,
        vaults: txn.root_db(Root::Vaults as usize)?,
        spaces: txn.root_db(Root::Spaces as usize)?,
        open_spaces: Mutex::new(HashMap::default()),
        txn,
        cur_space: None,
      })
    }

    if let Some(txn) = begin(txn) {
      Ok(txn)
    } else {
      Err(EncycError::PristineCorrupted)
    }
  }

  pub fn arc_txn_begin(&self) -> Result<ArcTxn<MutTxn<()>>, EncycError> {
    Ok(ArcTxn(Arc::new(RwLock::new(self.mut_txn_begin()?))))
  }

  pub fn mut_txn_begin(&self) -> Result<MutTxn<()>, EncycError> {
    let mut txn = Env::mut_txn_begin(self.env.clone())?;
    if let Some(version) = txn.root(Root::Version as usize) {
      if L64(version) != VERSION {
        return Err(EncycError::Version.into());
      }
    } else {
      txn.set_root(Root::Version as usize, VERSION.0);
    }

    Ok(MutTxn {
      entries: if let Some(db) = txn.root_db(Root::Entries as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      compartments: if let Some(db) = txn.root_db(Root::Compartments as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      labels: if let Some(db) = txn.root_db(Root::Labels as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      filters: if let Some(db) = txn.root_db(Root::Filters as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      vaults: if let Some(db) = txn.root_db(Root::Vaults as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      spaces: if let Some(db) = txn.root_db(Root::Spaces as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      open_spaces: Mutex::new(HashMap::default()),
      txn,
      cur_space: None,
    })
  }
}

pub type Txn = GenericTxn<sanakirja::Txn<Arc<Env>>>;
pub type MutTxn<T> = GenericTxn<sanakirja::MutTxn<Arc<Env>, T>>;

pub struct GenericTxn<T>
where
  T: LoadPage<Error = sanakirja::Error> + RootPage,
{
  #[doc(hidden)]
  pub txn: T,

  pub entries: UDb<ChangeId, SerializedEntry>,       // like transactions
  pub compartments: UDb<UId, SerializedCompartment>, // like accounts
  pub labels: UDb<UId, SerializedLabel>,             // like tags, but can be applied to any entry
  pub filters: UDb<UId, SerializedFilter>,           // like campaigns, budgets etc. (can be applied to any entry)

  pub vaults: UDb<UId, SerializedVault>,
  // open_vaults: Mutex<HashMap<UId, VaultRef<Self>>>,

  //
  pub(crate) open_spaces: Mutex<HashMap<SmallString, space::SpaceRef<Self>>>,
  pub(super) spaces: UDb<SmallStr, space::SerializedSpace>,
  pub(super) cur_space: Option<String>,
}

/// This is actually safe because the only non-Send fields are
/// `open_spaces`, but we can't do anything with
/// a `SpaceRef` whose transaction has been moved to another thread.
unsafe impl<T: LoadPage<Error = sanakirja::Error> + RootPage> Send for GenericTxn<T> {}

impl<T: LoadPage<Error = sanakirja::Error> + RootPage> TxnT for GenericTxn<T> {}

impl MutTxnT for MutTxn<()> {
  fn open_or_create_space(&mut self, name: &str) -> Result<space::SpaceRef<Self>, Self::GraphError> {
    todo!("self.open_or_create_space(name={})?", name)
    // let name = SmallString::from_str(name);
    // let mut commit = None;

    // let result = match self.open_spaces.lock().entry(name.clone()) {
    //   Entry::Vacant(v) => {
    //     let r = match btree::get(&self.txn, &self.spaces, &name, None)? {
    //       Some((name_, b)) if name_ == name.as_ref() => space::SpaceRef::new(space::Space {
    //         id: b.id,
    //         name: name.clone(),
    //         last_modified: b.last_modified,
    //         changes: Db::from_page(b.changes.into()),
    //         vaults: Db::from_page(b.vaults.into()),
    //       }),
    //       _ => {
    //         let br = space::SpaceRef::new(space::Space {
    //           id: UId::new(),
    //           name: name.clone(),
    //           last_modified: 0,
    //           changes: btree::create_db_(&mut self.txn)?,
    //           vaults: btree::create_db_(&mut self.txn)?,
    //         });
    //         commit = Some(br.clone());
    //         br
    //       }
    //     };
    //   }
    //   Entry::Occupied(occ) => todo!(),
    // };

    // if let Some(commit) = commit {
    //   todo!("self.put_space(&commit)?");
    // }

    // Ok(result)
  }

  fn commit(self) -> Result<(), Self::GraphError> {
    // use std::ops::DerefMut;

    // {
    //   let open_spaces = std::mem::replace(self.open_spaces.lock().deref_mut(), HashMap::default());
    //   for (name, space) in open_spaces {
    //     debug!("commit_space {:?}", name);
    //     todo!("self.commit_space(&space)?");
    //   }
    // }

    // if let Some(ref cur) = self.cur_space {
    //   unsafe {
    //     assert!(cur.len() < 256);
    //     let b = self.txn.root_page_mut();
    //     b[4096 - 256] = cur.len() as u8;
    //     std::ptr::copy(cur.as_ptr(), b.as_mut_ptr().add(4096 - 255), cur.len())
    //   }
    // }

    // debug!("{:x} {:x}", self.entries.db, self.spaces.db);

    // self.txn.set_root(Root::Entries as usize, self.entries.db);
    // self.txn.set_root(Root::Spaces as usize, self.spaces.db);

    todo!("self.commit()?")
  }
}
