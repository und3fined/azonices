// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

mod strings;
pub mod types;

mod prelude {
  use sanakirja::{direct_repr, Storable, UnsizedStorable};

  use crate::{
    pristine::{ChangeId, SerializedHash},
    types::UId,
  };

  // register sanakirja storage
  direct_repr!(UId);
  impl sanakirja::debug::Check for UId {}

  direct_repr!(SerializedHash);
  impl sanakirja::debug::Check for SerializedHash {}

  direct_repr!(ChangeId);
  impl sanakirja::debug::Check for ChangeId {}
}
