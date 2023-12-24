// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use anyhow::Result;
use std::{io::ErrorKind, path::Path, sync::Arc};

use sanakirja::Env;

use super::EncycError;

const DB_SIZE: u64 = 1 << 13; // 8KB

pub struct Encyc {
  pub env: Arc<Env>,
}

// Encyc is a short string for Encyclopedia and a wrapper around sanakirja::Env
impl Encyc {
  pub fn new<P: AsRef<Path>>(name: P) -> Result<Self, EncycError> {
    Self::new_with_size(name, DB_SIZE)
  }

  pub fn new_with_size<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, EncycError> {
    let env = Env::new(name, size, 2);
    match env {
      Ok(env) => Ok(Self { env: Arc::new(env) }),
      Err(sanakirja::Error::IO(e)) => {
        if e.kind() == ErrorKind::WouldBlock {
          Err(EncycError::PristineLocked)
        } else {
          Err(EncycError::Sanakirja(sanakirja::Error::IO(e)))
        }
      }
      Err(e) => Err(EncycError::Sanakirja(e)),
    }
  }

  pub unsafe fn new_nolock<P: AsRef<Path>>(name: P) -> Result<Self, EncycError> {
    Self::new_with_size_nolock(name, DB_SIZE)
  }

  pub unsafe fn new_with_size_nolock<P: AsRef<Path>>(name: P, size: u64) -> Result<Self, EncycError> {
    let env = Env::new_nolock(name, size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }

  pub fn new_anony() -> Result<Self, EncycError> {
    Self::new_anony_with_size(DB_SIZE)
  }

  pub fn new_anony_with_size(size: u64) -> Result<Self, EncycError> {
    let env = Env::new_anon(size, 2)?;
    Ok(Self { env: Arc::new(env) })
  }
}
