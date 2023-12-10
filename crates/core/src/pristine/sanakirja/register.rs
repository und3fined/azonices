// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

use sanakirja::{direct_repr, Storable, UnsizedStorable};

use crate::types::UId;

// register sanakirja storage
direct_repr!(UId);
impl sanakirja::debug::Check for UId {}
