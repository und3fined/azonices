// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 16.

mod prelude;
pub use prelude::*;

use sanakirja::{LoadPage, RootPage};

use crate::pristine::{EncycError, GenericTxn};

impl<T: LoadPage<Error = sanakirja::Error> + RootPage> GraphTxnT for GenericTxn<T> {
  type GraphError = EncycError;
}
