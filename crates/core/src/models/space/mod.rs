// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.
#![allow(dead_code)]

mod prelude;
pub use prelude::SpaceTxnT;

use std::sync::Arc;

use parking_lot::RwLock;
use sanakirja::{LoadPage, RootPage};

use crate::{
  pristine::GenericTxn,
  types::{SmallString, UId},
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

pub struct Space {
  pub id: UId,
  pub name: SmallString,
}

impl<T: LoadPage<Error = sanakirja::Error> + RootPage> SpaceTxnT for GenericTxn<T> {
  type Space = Space;

  fn id<'a>(&self, s: &'a Self::Space) -> Option<&'a UId> {
    Some(&s.id)
  }

  fn name<'a>(&self, space: &'a Self::Space) -> &'a str {
    space.name.as_str()
  }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct SerializedSpace {
  id: UId,
}
