// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 15.

use crate::models::{graph::GraphTxnT, space::SpaceTxnT};

pub trait TxnT: GraphTxnT + SpaceTxnT {}
