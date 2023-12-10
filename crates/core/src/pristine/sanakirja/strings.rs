// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

use log::debug;
use sanakirja::{debug::Check, Storable, UnsizedStorable};

use crate::types::SmallStr;

impl UnsizedStorable for SmallStr {
  const ALIGN: usize = 1;

  fn size(&self) -> usize {
    1 + self.len as usize
  }

  unsafe fn write_to_page(&self, p: *mut u8) {
    std::ptr::copy(&self.len, p, self.size());
    debug!(
      "Wrote SmallStr to page: {:?}",
      std::slice::from_raw_parts(p, self.size())
    );
  }

  unsafe fn from_raw_ptr<'a, T>(_: &T, p: *const u8) -> &'a Self {
    smallstr_from_raw_ptr(p)
  }

  unsafe fn onpage_size(p: *const u8) -> usize {
    let len = *p as usize;
    debug!(
      "onpage_size {:?}",
      std::slice::from_raw_parts(p, 1 + len as usize)
    );
    1 + len
  }
}

impl Storable for SmallStr {
  fn compare<T>(&self, _: &T, x: &Self) -> std::cmp::Ordering {
    self.cmp(x)
  }
  type PageReferences = std::iter::Empty<u64>;
  fn page_references(&self) -> Self::PageReferences {
    std::iter::empty()
  }
}

impl Check for SmallStr {}

unsafe fn smallstr_from_raw_ptr<'a>(p: *const u8) -> &'a SmallStr {
  let len = *p as usize;
  std::mem::transmute(std::slice::from_raw_parts(p, 1 + len as usize))
}
