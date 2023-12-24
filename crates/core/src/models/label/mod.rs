// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 24.
#![allow(dead_code)]

use crate::types::{SmallString, UId, L64};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum LabelGroup {
  INCOME,
  EXPENSE,
  DEBT,
  LOAN,
  TAG,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct SerializedLabel {
  header: L64,       // store owner, change, metadata
  group: LabelGroup, // store group like income, expense, etc.
  name: SmallString,
  id: UId,
}

pub struct Label {}
// #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
// #[repr(C)]
// pub struct SerializedChannel {
//     graph: L64,
//     changes: L64,
//     revchanges: L64,
//     states: L64,
//     tags: L64,
//     apply_counter: L64,
//     last_modified: L64,
//     id: RemoteId,
// }
// pub struct Channel {
//     pub graph: Db<Vertex<ChangeId>, SerializedEdge>,
//     pub changes: Db<ChangeId, L64>,
//     pub revchanges: UDb<L64, Pair<ChangeId, SerializedMerkle>>,
//     pub states: UDb<SerializedMerkle, L64>,
//     pub tags: Db<L64, Pair<SerializedMerkle, SerializedMerkle>>,
//     pub apply_counter: ApplyTimestamp,
//     pub name: SmallString,
//     pub last_modified: u64,
//     pub id: RemoteId,
// }
