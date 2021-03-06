// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;

use ffi;

#[repr(C)]
pub struct SDPKey(ffi::GstSDPKey);

impl SDPKey {
    pub fn type_(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.type_).to_str().unwrap() }
    }

    pub fn data(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.data).to_str().unwrap() }
    }
}

impl fmt::Debug for SDPKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPKey")
            .field("type", &self.type_())
            .field("data", &self.data())
            .finish()
    }
}
