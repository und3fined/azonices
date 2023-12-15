// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 14.
#![allow(dead_code)]

use lazy_static::lazy_static;

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
