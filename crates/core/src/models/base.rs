// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.
#![allow(dead_code)]

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author(pub BTreeMap<String, String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
  pub name: String,
  pub alias: Option<String>,
  pub created_at: DateTime<Utc>,
  pub last_access: DateTime<Utc>,
  pub raw: Option<String>,
  pub platform: Option<String>,
  pub os: Option<String>,
  pub os_version: Option<String>,
}

impl Default for Device {
  fn default() -> Self {
    Self {
      name: String::new(),
      alias: None,
      created_at: Utc::now(),
      last_access: Utc::now(),
      raw: None,
      platform: None,
      os: None,
      os_version: None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChangeHeader_<A, D> {
  pub timestamp: DateTime<Utc>,
  pub authors: Vec<A>,
  pub devices: Vec<D>,
}

pub type ChangeHeader = ChangeHeader_<Author, Device>;

impl Default for ChangeHeader {
  fn default() -> Self {
    Self {
      timestamp: Utc::now(),
      authors: Vec::new(),
      devices: Vec::new(),
    }
  }
}
