// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 19.

pub trait Base32: Sized {
  fn to_base32(&self) -> String;
  fn from_base32(b: &[u8]) -> Option<Self>;
}
