// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 16.

pub trait GraphTxnT: Sized {
  type GraphError: std::error::Error + std::fmt::Debug + Send + Sync + 'static;
}
