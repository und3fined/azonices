// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.

use crate::models::space::SpaceRef;

use super::*;

pub trait MutTxnT: TxnT {
  fn commit(self) -> Result<(), Self::GraphError>;
  fn open_or_create_space(&mut self, name: &str) -> Result<SpaceRef<Self>, Self::GraphError>;
}
