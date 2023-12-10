// Copyright (c) 2023 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2023 Dec 10.

pub const MAX_LEN: usize = 255;

/// A string of length at most 255 bytes, with a more compact on-disk encoding.
pub struct SmallString {
  pub len: u8,
  pub str: [u8; MAX_LEN],
}

pub struct SmallStr {
  pub len: u8,
  _str: [u8],
}

impl Default for SmallString {
  fn default() -> Self {
    Self {
      len: 0,
      str: [0; MAX_LEN],
    }
  }
}

impl SmallString {
  pub fn new() -> Self {
    Self::default()
  }
  /// ```ignore
  /// use libpijul::small_string::*;
  /// let mut s = SmallString::from_str("blah!");
  /// assert_eq!(s.len(), s.as_str().len());
  /// ```
  pub fn len(&self) -> usize {
    self.len as usize
  }

  /// ```ignore
  /// use libpijul::small_string::*;
  /// let mut s = SmallString::from_str("blah");
  /// s.clear();
  /// assert_eq!(s.as_str(), "");
  /// assert!(s.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn from_str(s: &str) -> Self {
    let mut b = SmallString {
      len: s.len() as u8,
      str: [0; MAX_LEN],
    };
    b.clone_from_str(s);
    b
  }
  pub fn clone_from_str(&mut self, s: &str) {
    self.len = s.len() as u8;
    (&mut self.str[..s.len()]).copy_from_slice(s.as_bytes());
  }

  /// ```ignore
  /// use libpijul::small_string::*;
  /// let mut s = SmallString::from_str("blah");
  /// s.clear();
  /// assert!(s.is_empty());
  /// ```
  pub fn clear(&mut self) {
    self.len = 0;
  }
  pub fn push_str(&mut self, s: &str) {
    let l = self.len as usize;
    assert!(l + s.len() <= 0xff);
    (&mut self.str[l..l + s.len()]).copy_from_slice(s.as_bytes());
    self.len += s.len() as u8;
  }

  pub fn as_str(&self) -> &str {
    use std::ops::Deref;
    self.deref().as_str()
  }

  pub fn as_bytes(&self) -> &[u8] {
    use std::ops::Deref;
    self.deref().as_bytes()
  }
}

impl PartialEq for SmallString {
  fn eq(&self, x: &SmallString) -> bool {
    self.as_str().eq(x.as_str())
  }
}

impl Eq for SmallString {}

impl PartialOrd for SmallString {
  fn partial_cmp(&self, x: &SmallString) -> Option<std::cmp::Ordering> {
    self.as_str().partial_cmp(x.as_str())
  }
}
impl Ord for SmallString {
  fn cmp(&self, x: &SmallString) -> std::cmp::Ordering {
    self.as_str().cmp(x.as_str())
  }
}

impl std::hash::Hash for SmallString {
  fn hash<H: std::hash::Hasher>(&self, x: &mut H) {
    self.as_str().hash(x)
  }
}

impl Clone for SmallString {
  fn clone(&self) -> Self {
    Self::from_str(self.as_str())
  }
}

impl std::fmt::Debug for SmallString {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::ops::Deref;
    self.deref().fmt(fmt)
  }
}

impl std::ops::Deref for SmallString {
  type Target = SmallStr;
  fn deref(&self) -> &Self::Target {
    let len = self.len as usize;
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const Self as *const u8,
        1 + len,
      ))
    }
  }
}

impl AsRef<SmallStr> for SmallString {
  fn as_ref(&self) -> &SmallStr {
    let len = self.len as usize;
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts(
        self as *const Self as *const u8,
        1 + len,
      ))
    }
  }
}

impl AsMut<SmallStr> for SmallString {
  fn as_mut(&mut self) -> &mut SmallStr {
    let len = self.len as usize;
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut Self as *mut u8,
        1 + len,
      ))
    }
  }
}

impl std::ops::DerefMut for SmallString {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let len = self.len as usize;
    unsafe {
      std::mem::transmute(std::slice::from_raw_parts_mut(
        self as *mut Self as *mut u8,
        1 + len,
      ))
    }
  }
}

impl SmallStr {
  /// ```ignore
  /// use libpijul::small_string::*;
  /// let mut s = SmallString::from_str("");
  /// assert!(s.as_small_str().is_empty());
  /// s.push_str("blah");
  /// assert!(!s.as_small_str().is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// ```ignore
  /// use libpijul::small_string::*;
  /// let mut s = SmallString::from_str("blah");
  /// assert_eq!(s.as_small_str().len(), "blah".len())
  /// ```
  pub fn len(&self) -> usize {
    self.len as usize
  }

  pub fn as_str(&self) -> &str {
    unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
  }

  pub fn as_bytes(&self) -> &[u8] {
    let s: &[u8] = unsafe { std::mem::transmute(self) };
    &s[1..]
  }

  pub fn to_owned(&self) -> SmallString {
    SmallString::from_str(self.as_str())
  }
}

impl std::hash::Hash for SmallStr {
  fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
    self.as_bytes().hash(hasher)
  }
}

impl PartialEq for SmallStr {
  fn eq(&self, x: &SmallStr) -> bool {
    self.as_str().eq(x.as_str())
  }
}

impl Eq for SmallStr {}

impl PartialOrd for SmallStr {
  fn partial_cmp(&self, x: &SmallStr) -> Option<std::cmp::Ordering> {
    self.as_str().partial_cmp(x.as_str())
  }
}

impl Ord for SmallStr {
  fn cmp(&self, x: &SmallStr) -> std::cmp::Ordering {
    self.as_str().cmp(x.as_str())
  }
}

impl std::fmt::Debug for SmallStr {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.as_str().fmt(fmt)
  }
}
