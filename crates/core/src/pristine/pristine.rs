// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use anyhow::Result;
use std::{io::ErrorKind, path::Path, sync::Arc};

use sanakirja::Env;

use super::SajaError;

const DB_SIZE: u64 = 1 << 13; // 8KB

pub struct Pristine {
  pub env: Arc<Env>,
}

// Prinstine is a wrapper around sanakirja::Env
impl Pristine {
  pub fn new<P: AsRef<Path>>(name: P) -> Result<Self, SajaError> {
    Self::new_with_size(name, DB_SIZE)
  }

  pub fn new_with_size<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, SajaError> {
    let env = Env::new(name, size, 2);
    match env {
      Ok(env) => Ok(Self { env: Arc::new(env) }),
      Err(sanakirja::Error::IO(e)) => {
        if e.kind() == ErrorKind::WouldBlock {
          Err(SajaError::PristineLocked)
        } else {
          Err(SajaError::Sanakirja(sanakirja::Error::IO(e)))
        }
      }
      Err(e) => Err(SajaError::Sanakirja(e)),
    }
  }

  pub unsafe fn new_nolock<P: AsRef<Path>>(name: P) -> Result<Self, SajaError> {
    Self::new_with_size_nolock(name, DB_SIZE)
  }

  pub unsafe fn new_with_size_nolock<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, SajaError> {
    let env = Env::new_nolock(name, size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }

  pub fn new_anony() -> Result<Self, SajaError> {
    Self::new_anony_with_size(DB_SIZE)
  }

  pub fn new_anony_with_size(size: u64) -> Result<Self, SajaError> {
    let env = Env::new_anon(size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }
}
