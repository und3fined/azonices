// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.
#![allow(dead_code)]

mod prelude;
pub use prelude::SpaceTxnT;

use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use sanakirja::{LoadPage, RootPage};

use crate::{
  pristine::{types::Db, GenericTxn},
  types::{ChangeId, SmallString, UId, L64},
};

pub struct SpaceRef<T: SpaceTxnT> {
  pub(crate) r: Arc<RwLock<T::Space>>,
}

impl<T: SpaceTxnT> SpaceRef<T> {
  pub fn new(t: T::Space) -> Self {
    SpaceRef {
      r: Arc::new(RwLock::new(t)),
    }
  }
}

impl<T: SpaceTxnT> SpaceRef<T> {
  pub fn read(&self) -> RwLockReadGuard<T::Space> {
    self.r.read()
  }
  pub fn write(&self) -> RwLockWriteGuard<T::Space> {
    self.r.write()
  }
}

impl<T: SpaceTxnT> Clone for SpaceRef<T> {
  fn clone(&self) -> Self {
    SpaceRef { r: self.r.clone() }
  }
}

pub struct Space {
  pub id: UId,
  pub name: SmallString,
  pub last_modified: u64,

  pub changes: Db<ChangeId, L64>,
  pub vaults: Db<UId, L64>,
}

impl<T: LoadPage<Error = sanakirja::Error> + RootPage> SpaceTxnT for GenericTxn<T> {
  type Space = Space;

  fn id<'a>(&self, s: &'a Self::Space) -> Option<&'a UId> {
    Some(&s.id)
  }

  fn name<'a>(&self, space: &'a Self::Space) -> &'a str {
    space.name.as_str()
  }

  type Changeset = Db<ChangeId, L64>;
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct SerializedSpace {
  pub id: UId,
  pub changes: L64,
  pub vaults: L64,
  pub last_modified: u64,
}
