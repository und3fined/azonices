// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
  pub(crate) static ref BASE32: data_encoding::Encoding = {
    let mut spec = data_encoding::Specification::new();
    spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
    spec.translate.from = "abcdefghijklmnopqrstuvwxyz".to_string();
    spec.translate.to = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    spec.encoding().unwrap()
  };
}

pub trait Base32: Sized {
  fn to_base32(&self) -> String;
  fn from_base32(b: &[u8]) -> Option<Self>;
}

pub struct ArcTxn<T>(pub Arc<RwLock<T>>);

impl<T> ArcTxn<T> {
  pub fn new(t: T) -> Self {
    ArcTxn(Arc::new(RwLock::new(t)))
  }
}

impl<T> Clone for ArcTxn<T> {
  fn clone(&self) -> Self {
    ArcTxn(self.0.clone())
  }
}

// impl<T: MutTxnT> ArcTxn<T> {
//   pub fn commit(self) -> Result<(), T::GraphError> {
//     if let Ok(txn) = Arc::try_unwrap(self.0) {
//       txn.into_inner().commit()
//     } else {
//       panic!("Tried to commit an ArcTxn without dropping its references")
//     }
//   }
// }

impl<T> std::ops::Deref for ArcTxn<T> {
  type Target = RwLock<T>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
