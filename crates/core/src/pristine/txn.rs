// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use sanakirja::{Env, LoadPage, RootPage};

use crate::{
  models::space,
  traits::MutTxnT,
  types::{HashMap, SmallString},
};

use super::{hash::*, sanakirja::types::*, ChangeId};

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

// impl<T: MutTxnT> ArcTxn<T> {
//   pub fn commit(self) -> Result<(), T::GraphError> {
//     if let Ok(txn) = Arc::try_unwrap(self.0) {
//       txn.into_inner().commit()
//     } else {
//       panic!("Tried to commit an ArcTxn without dropping its references")
//     }
//   }
// }

impl<T> std::ops::Deref for ArcTxn<T> {
  type Target = RwLock<T>;
  fn deref(&self) -> &Self::Target {
    &self.0
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

  // transaction saved here
  #[doc(hidden)]
  pub internal: UDb<SerializedHash, ChangeId>,
  #[doc(hidden)]
  pub external: UDb<ChangeId, SerializedHash>,

  pub(crate) open_spaces: Mutex<HashMap<SmallString, space::SpaceRef<Self>>>,

  pub(super) spaces: UDb<SmallString, space::SerializedSpace>,

  pub(super) counter: usize,
  pub(super) cur_space: Option<String>,
}
