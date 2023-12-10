// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use super::BASE32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UId(pub [u8; 16]);

impl UId {
  pub fn nil() -> Self {
    UId([0; 16])
  }

  pub fn from_bytes(b: &[u8]) -> Option<Self> {
    if b.len() != 16 {
      return None;
    }
    let mut x = UId([0; 16]);
    unsafe {
      std::ptr::copy_nonoverlapping(b.as_ptr(), x.0.as_mut_ptr(), 16);
    }

    Some(x)
  }

  pub fn as_bytes(&self) -> &[u8; 16] {
    &self.0
  }

  pub fn from_base32(b: &[u8]) -> Option<Self> {
    let mut bb = UId([0; 16]);
    if b.len() != BASE32.encode_len(16) {
      return None;
    }

    if BASE32.decode_mut(b, &mut bb.0).is_ok() {
      Some(bb)
    } else {
      None
    }
  }

  pub fn new() -> Self {
    let mut rng = rand::thread_rng();
    use rand::Rng;
    let mut x = UId([0; 16]);
    for x in x.0.iter_mut() {
      *x = rng.gen();
    }
    x
  }
}

impl Display for UId {
  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
    write!(fmt, "{}", BASE32.encode(&self.0))
  }
}

impl std::fmt::Debug for UId {
  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
    write!(fmt, "{}", BASE32.encode(&self.0))
  }
}
