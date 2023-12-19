// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 19.

use curve25519_dalek::{constants::ED25519_BASEPOINT_POINT, edwards, scalar};
use serde::{Deserialize, Serialize};

use crate::ParseError;

use super::{Base32, Hash, BASE32};

pub(crate) const BASE32_BYTES: usize = 53;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Merkle {
  Ed25519(edwards::EdwardsPoint),
}

impl Default for Merkle {
  fn default() -> Self {
    Merkle::zero()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MerkleAlgorithm {
  Ed25519 = 1,
}

impl Merkle {
  pub fn zero() -> Self {
    Merkle::Ed25519(ED25519_BASEPOINT_POINT)
  }

  pub fn next<S: Into<scalar::Scalar>>(&self, h: S) -> Self {
    match self {
      Merkle::Ed25519(ref h0) => {
        let s = h.into(); // scalar
        Merkle::Ed25519(h0 * s)
      }
    }
  }

  pub fn to_bytes(&self) -> [u8; 32] {
    match *self {
      Merkle::Ed25519(ref e) => e.compress().to_bytes(),
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

    match edwards::CompressedEdwardsY::from_slice(&bytes[..32]) {
      Ok(x) => x.decompress().map(Merkle::Ed25519),
      Err(_) => None,
    }
  }
}

impl Base32 for Merkle {
  fn to_base32(&self) -> String {
    match *self {
      Merkle::Ed25519(ref s) => {
        let mut b32 = [0; 33];
        (&mut b32[..32]).clone_from_slice(s.compress().as_bytes());
        b32[32] = MerkleAlgorithm::Ed25519 as u8;
        BASE32.encode(&b32)
      }
    }
  }

  fn from_base32(b: &[u8]) -> Option<Self> {
    let bytes = if let Ok(bytes) = BASE32.decode(b) {
      bytes
    } else {
      return None;
    };

    if bytes.len() == 33 && *bytes.last().unwrap() == MerkleAlgorithm::Ed25519 as u8 {
      match edwards::CompressedEdwardsY::from_slice(&bytes[..32]) {
        Ok(x) => x.decompress().map(Merkle::Ed25519),
        Err(_) => None,
      }
    } else {
      None
    }
  }
}

impl std::fmt::Debug for Merkle {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "{:?}", self.to_base32())
  }
}

impl std::hash::Hash for Merkle {
  fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
    match self {
      Merkle::Ed25519(x) => x.compress().as_bytes().hash(hasher),
    }
  }
}

impl std::str::FromStr for Merkle {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(x) = Self::from_base32(s.as_bytes()) {
      Ok(x)
    } else {
      Err(ParseError { s: s.to_string() })
    }
  }
}

impl From<&Hash> for scalar::Scalar {
  fn from(h: &Hash) -> Self {
    match h {
      Hash::Blake3(h) => scalar::Scalar::from_bytes_mod_order(*h),
      _ => unreachable!(),
    }
  }
}

impl From<&Merkle> for scalar::Scalar {
  fn from(value: &Merkle) -> Self {
    match *value {
      Merkle::Ed25519(x) => {
        let x = x.compress();
        scalar::Scalar::from_bytes_mod_order(*x.as_bytes())
      }
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedMerkle(pub [u8; 33]);

impl PartialEq<Merkle> for SerializedMerkle {
  fn eq(&self, other: &Merkle) -> bool {
    match other {
      Merkle::Ed25519(q) => {
        (self.0)[0] == MerkleAlgorithm::Ed25519 as u8 && {
          let q = q.compress();
          q.as_bytes() == &(self.0)[1..]
        }
      }
    }
  }
}

impl PartialEq<SerializedMerkle> for Merkle {
  fn eq(&self, other: &SerializedMerkle) -> bool {
    other.eq(self)
  }
}

impl<'a> From<&'a Merkle> for SerializedMerkle {
  fn from(value: &'a Merkle) -> Self {
    let mut mm = [0; 33];
    match value {
      Merkle::Ed25519(x) => {
        mm[0] = MerkleAlgorithm::Ed25519 as u8;
        let x = x.compress();
        let x = x.as_bytes();
        (&mut mm[1..]).clone_from_slice(x);
        SerializedMerkle(mm)
      }
    }
  }
}

impl From<Merkle> for SerializedMerkle {
  fn from(m: Merkle) -> Self {
    let mut mm = [0; 33];
    match m {
      Merkle::Ed25519(q) => {
        mm[0] = MerkleAlgorithm::Ed25519 as u8;
        let q = q.compress();
        let q = q.as_bytes();
        (&mut mm[1..]).copy_from_slice(q);
        SerializedMerkle(mm)
      }
    }
  }
}

impl<'a> From<&'a SerializedMerkle> for Merkle {
  fn from(m: &'a SerializedMerkle) -> Self {
    assert_eq!((m.0)[0], MerkleAlgorithm::Ed25519 as u8);
    Merkle::Ed25519(match edwards::CompressedEdwardsY::from_slice(&(m.0)[1..]) {
      Ok(x) => x.decompress().unwrap(),
      _ => unreachable!(),
    })
  }
}

impl From<SerializedMerkle> for Merkle {
  fn from(m: SerializedMerkle) -> Self {
    assert_eq!((m.0)[0], MerkleAlgorithm::Ed25519 as u8);
    Merkle::Ed25519(match edwards::CompressedEdwardsY::from_slice(&(m.0)[1..]) {
      Ok(x) => x.decompress().unwrap(),
      _ => unreachable!(),
    })
  }
}

impl std::fmt::Debug for SerializedMerkle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Merkle::from(self).fmt(f)
  }
}
