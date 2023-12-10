// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

mod l64;
pub use l64::*;
mod uid;
pub use uid::*;
mod strings;
pub use strings::*;

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
