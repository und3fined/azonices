// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

use std::path::PathBuf;

use anyhow::bail;

pub fn current_dir() -> Result<PathBuf, anyhow::Error> {
  if let Ok(cur) = std::env::current_dir() {
    Ok(cur)
  } else {
    bail!("Cannot access working directory")
  }
}
