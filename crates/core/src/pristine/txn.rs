// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use sanakirja::{Env, LoadPage, RootPage};

use crate::{
  models::space::SpaceRef,
  types::{HashMap, SmallString},
};

use super::{hash::*, sanakirja::types::*, ChangeId};

pub type Txn = GenericTxn<sanakirja::Txn<Arc<Env>>>;
pub type MutTxn<T> = GenericTxn<sanakirja::MutTxn<Arc<Env>, T>>;

pub struct GenericTxn<T>
where
  T: LoadPage<Error = sanakirja::Error> + RootPage,
{
  #[doc(hidden)]
  pub txn: T,

  // transaction saved here
  #[doc(hidden)]
  pub internal: UDb<SerializedHash, ChangeId>,
  #[doc(hidden)]
  pub external: UDb<ChangeId, SerializedHash>,

  pub(crate) open_spaces: Mutex<HashMap<SmallString, SpaceRef<Self>>>,
  counter: usize,
  cur_space: Option<String>,
}
