// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.

use sanakirja::{direct_repr, Storable, UnsizedStorable};

use crate::{
  models::{compartment::SerializedCompartment, entry::SerializedEntry, filter::SerializedFilter, label::SerializedLabel, space::SerializedSpace},
  types::{ChangeId, SerializedHash, UId, L64},
};

// register sanakirja storage
direct_repr!(UId);
impl sanakirja::debug::Check for UId {}

direct_repr!(L64);
impl sanakirja::debug::Check for L64 {}

direct_repr!(SerializedHash);
impl sanakirja::debug::Check for SerializedHash {}

direct_repr!(ChangeId);
impl sanakirja::debug::Check for ChangeId {}

direct_repr!(SerializedSpace);
impl sanakirja::debug::Check for SerializedSpace {}

// register model storage
direct_repr!(SerializedEntry);
impl sanakirja::debug::Check for SerializedEntry {}

// register model compartment storage
direct_repr!(SerializedCompartment);
impl sanakirja::debug::Check for SerializedCompartment {}

direct_repr!(SerializedFilter);
impl sanakirja::debug::Check for SerializedFilter {}

direct_repr!(SerializedLabel);
impl sanakirja::debug::Check for SerializedLabel {}
