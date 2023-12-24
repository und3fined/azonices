// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 25.

mod prelude;
pub use prelude::VaultTxnT;
use sanakirja::{LoadPage, RootPage};

use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
  pristine::{types::UDb, GenericTxn},
  types::{SmallString, UId, L64},
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct SerializedVault {
  compartments: L64,
  labels: L64,

  mode: u8,
  alias: SmallString,
  name: SmallString, // name of the vault
  id: UId,
}

pub struct Vault<T: VaultTxnT> {
  pub compartments: T::Compartments,
  pub labels: T::Labels,

  pub mode: u8,
  pub alias: SmallString,
  pub name: SmallString, // name of the vault
  pub id: UId,
}

pub struct VaultRef<T: VaultTxnT> {
  db: Arc<Mutex<Vault<T>>>,
  id: UId,
}

impl<T: VaultTxnT> Clone for VaultRef<T> {
  fn clone(&self) -> Self {
    Self {
      db: self.db.clone(),
      id: self.id.clone(),
    }
  }
}

impl<T: LoadPage<Error = sanakirja::Error> + RootPage> VaultTxnT for GenericTxn<T> {
  type Labels = UDb<L64, UId>;
  type Compartments = UDb<L64, UId>;
}
