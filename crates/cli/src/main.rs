// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

use std::fs;

use anyhow::Result;
use azoni_core::pristine;
use azoni_x::path::current_dir;

const DOT_DIR: &str = ".azoni";

fn main() -> Result<()> {
  let cur = current_dir().unwrap();
  let azoni_dir = cur.join(DOT_DIR);

  if fs::metadata(&azoni_dir).is_err() {
    fs::create_dir(&azoni_dir).unwrap();
  }

  let _encyc = pristine::Encyc::new(&azoni_dir.join("azoni.db"))?;

  println!("Current directory: {}", cur.display());

  Ok(())
}
