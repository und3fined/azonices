// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

use thiserror_impl::Error;

#[derive(Debug, Error)]
pub enum SajaError {
  #[error(transparent)]
  Sanakirja(#[from] sanakirja::Error),
  #[error("Pristine locked")]
  PristineLocked,
  #[error("Pristine corrupted")]
  PristineCorrupted,
  #[error(transparent)]
  Borrow(#[from] std::cell::BorrowError),
  #[error("Pristine version mismatch. Cloning over the network can fix this.")]
  Version,
}
