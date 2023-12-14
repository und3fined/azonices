// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.

use crate::types::UId;

pub trait SpaceTxnT {
  type Space: Sync + Send;

  fn id<'a>(&self, space: &'a Self::Space) -> Option<&'a UId>;
  fn name<'a>(&self, space: &'a Self::Space) -> &'a str;
}
