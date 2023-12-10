// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use sanakirja::btree;

pub(crate) type P<K, V> = btree::page::Page<K, V>;
pub type Db<K, V> = btree::Db<K, V>;
pub(crate) type UP<K, V> = btree::page_unsized::Page<K, V>;
pub type UDb<K, V> = btree::Db_<K, V, UP<K, V>>;
