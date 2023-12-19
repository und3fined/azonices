// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 20.

use crate::types::{UId, L64};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct SerializedEntry {
  pub id: UId,

  // DB fields
  pub changes: L64,
  pub tags: L64,

  // readable fields
  pub last_modified: L64,
  pub change_count: L64,
}
