// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use anyhow::Result;
use parking_lot::{Mutex, RwLock};
use std::{io::ErrorKind, path::Path, sync::Arc};

use sanakirja::{btree, Env, RootDb};

use crate::{
  pristine::{Root, VERSION},
  types::{HashMap, L64},
};

use super::{ArcTxn, EncycError, MutTxn, Txn};

const DB_SIZE: u64 = 1 << 13; // 8KB

pub struct Encyc {
  pub env: Arc<Env>,
}

// Encyc is a short string for Encyclopedia and a wrapper around sanakirja::Env
impl Encyc {
  pub fn new<P: AsRef<Path>>(name: P) -> Result<Self, EncycError> {
    Self::new_with_size(name, DB_SIZE)
  }

  pub fn new_with_size<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, EncycError> {
    let env = Env::new(name, size, 2);
    match env {
      Ok(env) => Ok(Self { env: Arc::new(env) }),
      Err(sanakirja::Error::IO(e)) => {
        if e.kind() == ErrorKind::WouldBlock {
          Err(EncycError::PristineLocked)
        } else {
          Err(EncycError::Sanakirja(sanakirja::Error::IO(e)))
        }
      }
      Err(e) => Err(EncycError::Sanakirja(e)),
    }
  }

  pub unsafe fn new_nolock<P: AsRef<Path>>(name: P) -> Result<Self, EncycError> {
    Self::new_with_size_nolock(name, DB_SIZE)
  }

  pub unsafe fn new_with_size_nolock<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, EncycError> {
    let env = Env::new_nolock(name, size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }

  pub fn new_anony() -> Result<Self, EncycError> {
    Self::new_anony_with_size(DB_SIZE)
  }

  pub fn new_anony_with_size(size: u64) -> Result<Self, EncycError> {
    let env = Env::new_anon(size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }
}

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
        spaces: txn.root_db(Root::Spaces as usize)?,
        open_spaces: Mutex::new(HashMap::default()),
        txn,
        counter: 0,
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
      spaces: if let Some(db) = txn.root_db(Root::Spaces as usize) {
        db
      } else {
        btree::create_db_(&mut txn)?
      },
      open_spaces: Mutex::new(HashMap::default()),
      txn,
      counter: 0,
      cur_space: None,
    })
  }
}
