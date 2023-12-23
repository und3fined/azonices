// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use std::{
  cmp::Ordering,
  fmt::{self, Formatter},
};

use serde::{Deserialize, Serialize};

use super::{Base32, BASE32};

pub(crate) const BLAKE3_BYTES: usize = 32;
pub(crate) const BASE32_BYTES: usize = 53;

#[derive(Serialize, Deserialize, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Hash {
  // None is the hash of the "null change"
  None,
  Blake3([u8; BLAKE3_BYTES]),
}

/// Algorithm used to compute change hashes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum HashAlgorithm {
  None = 0,
  Blake3 = 1,
}

impl Hash {
  pub fn to_bytes(&self) -> [u8; 1 + BLAKE3_BYTES] {
    match self {
      Hash::None => unimplemented!(),
      Hash::Blake3(ref hash) => {
        let mut out = [0; 1 + BLAKE3_BYTES];
        out[0] = HashAlgorithm::Blake3 as u8;
        (&mut out[1..]).clone_from_slice(hash);
        out
      }
    }
  }

  pub fn from_bytes(s: &[u8]) -> Option<Self> {
    if s.len() >= 1 + BLAKE3_BYTES && s[0] == HashAlgorithm::Blake3 as u8 {
      let mut out = [0; BLAKE3_BYTES];
      out.clone_from_slice(&s[1..]);
      Some(Hash::Blake3(out))
    } else {
      None
    }
  }

  pub fn from_prefix(s: &str) -> Option<Self> {
    let mut b32 = [b'A'; BASE32_BYTES];
    if s.len() > BASE32_BYTES {
      return None;
    }
    (&mut b32[..s.len()]).clone_from_slice(s.as_bytes());
    let bytes = if let Ok(bytes) = BASE32.decode(&b32) {
      bytes
    } else {
      return None;
    };
    let mut hash = [0; BLAKE3_BYTES];
    hash.clone_from_slice(&bytes[..BLAKE3_BYTES]);
    Some(Hash::Blake3(hash))
  }
}

impl Base32 for Hash {
  fn to_base32(&self) -> String {
    match *self {
      Hash::None => BASE32.encode(&[0]),
      Hash::Blake3(ref hash) => {
        let mut b3 = [0; 1 + BLAKE3_BYTES];
        b3[BLAKE3_BYTES] = HashAlgorithm::Blake3 as u8;
        (&mut b3[..BLAKE3_BYTES]).clone_from_slice(hash);
        BASE32.encode(&b3)
      }
    }
  }

  fn from_base32(s: &[u8]) -> Option<Self> {
    let bytes = if let Ok(s) = BASE32.decode(s) {
      s
    } else {
      return None;
    };

    if bytes == [0] {
      Some(Hash::None)
    } else if bytes.len() == 1 + BLAKE3_BYTES && bytes[BLAKE3_BYTES] == HashAlgorithm::Blake3 as u8 {
      let mut hash = [0; BLAKE3_BYTES];
      hash.clone_from_slice(&bytes[..BLAKE3_BYTES]);
      Some(Hash::Blake3(hash))
    } else {
      None
    }
  }
}

impl std::str::FromStr for Hash {
  type Err = crate::ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(b) = Self::from_base32(s.as_bytes()) {
      Ok(b)
    } else {
      Err(crate::ParseError { s: s.to_string() })
    }
  }
}

pub enum Hasher {
  Blake3(blake3::Hasher),
}

impl Default for Hasher {
  fn default() -> Self {
    Hasher::Blake3(blake3::Hasher::new())
  }
}

impl Hasher {
  pub fn update(&mut self, data: &[u8]) {
    match self {
      Hasher::Blake3(ref mut h) => {
        h.update(data);
      }
    }
  }

  pub fn finish(&self) -> Hash {
    match self {
      Hasher::Blake3(ref h) => {
        let result = h.finalize();
        let mut hash = [0; BLAKE3_BYTES];
        hash.clone_from_slice(result.as_bytes());
        Hash::Blake3(hash)
      }
    }
  }
}

impl fmt::Debug for Hash {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "{}", self.to_base32())
  }
}

#[derive(Clone, Copy)]
pub(crate) union H {
  none: (),
  blake3: [u8; BLAKE3_BYTES],
}

#[derive(Clone, Copy)]
pub struct SerializedHash {
  pub(crate) t: u8,
  h: H,
}

impl std::hash::Hash for SerializedHash {
  fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
    self.t.hash(hasher);
    if self.t == HashAlgorithm::Blake3 as u8 {
      unsafe { self.h.blake3.hash(hasher) }
    }
  }
}

pub(crate) const HASH_NONE: SerializedHash = SerializedHash {
  t: HashAlgorithm::None as u8,
  h: H { none: () },
};

impl PartialOrd for SerializedHash {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.t.cmp(&other.t))
  }
}

impl Ord for SerializedHash {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.t.cmp(&other.t) {
      Ordering::Equal => {
        if self.t == HashAlgorithm::Blake3 as u8 {
          unsafe { self.h.blake3.cmp(&other.h.blake3) }
        } else {
          Ordering::Equal
        }
      }
      o => o,
    }
  }
}

impl PartialEq for SerializedHash {
  fn eq(&self, other: &Self) -> bool {
    if self.t == HashAlgorithm::Blake3 as u8 && self.t == other.t {
      unsafe { self.h.blake3 == other.h.blake3 }
    } else {
      self.t == other.t
    }
  }
}

impl Eq for SerializedHash {}

impl std::fmt::Debug for SerializedHash {
  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
    Hash::from(self).fmt(fmt)
  }
}

impl<'a> From<&'a SerializedHash> for Hash {
  fn from(value: &'a SerializedHash) -> Self {
    if value.t == HashAlgorithm::Blake3 as u8 {
      Hash::Blake3(unsafe { value.h.blake3.clone() })
    } else if value.t == HashAlgorithm::None as u8 {
      Hash::None
    } else {
      panic!("Unknown hash algorithm {:?}", value.t)
    }
  }
}

impl From<SerializedHash> for Hash {
  fn from(value: SerializedHash) -> Self {
    (&value).into()
  }
}

impl<'a> From<&'a Hash> for SerializedHash {
  fn from(value: &'a Hash) -> Self {
    match value {
      Hash::Blake3(value) => SerializedHash {
        t: HashAlgorithm::Blake3 as u8,
        h: H {
          blake3: value.clone(),
        },
      },
      Hash::None => SerializedHash {
        t: 0,
        h: H { none: () },
      },
    }
  }
}

impl From<Hash> for SerializedHash {
  fn from(value: Hash) -> Self {
    (&value).into()
  }
}

impl SerializedHash {
  pub fn size(b: &[u8]) -> usize {
    if b[0] == HashAlgorithm::Blake3 as u8 {
      1 + BLAKE3_BYTES
    } else if b[0] == HashAlgorithm::None as u8 {
      1
    } else {
      panic!("Unknown hash algorithm {:?}", b[0])
    }
  }

  pub unsafe fn size_from_ptr(b: *const u8) -> usize {
    if *b == HashAlgorithm::Blake3 as u8 {
      1 + BLAKE3_BYTES
    } else if *b == HashAlgorithm::None as u8 {
      1
    } else {
      panic!("Unknown hash algorithm {:?}", *b)
    }
  }
}
