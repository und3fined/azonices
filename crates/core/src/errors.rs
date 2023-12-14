// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.

use log::error;
use thiserror_impl::Error;

#[derive(Debug, Error)]
#[error("Parse error: {:?}", s)]
pub struct ParseError {
  pub(crate) s: String,
}
