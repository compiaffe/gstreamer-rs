// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_char;
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use ffi;
use gobject_ffi;

use glib::translate::{from_glib, ToGlib, ToGlibPtr};
use glib::IsA;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DebugCategory(ptr::NonNull<ffi::GstDebugCategory>);

impl DebugCategory {
    pub fn new<'a, P: Into<Option<&'a str>>>(
        name: &str,
        color: ::DebugColorFlags,
        description: P,
    ) -> DebugCategory {
        extern "C" {
            fn _gst_debug_category_new(
                name: *const c_char,
                color: ffi::GstDebugColorFlags,
                description: *const c_char,
            ) -> *mut ffi::GstDebugCategory;
        }
        let description = description.into();

        // Gets the category if it exists already
        unsafe {
            let ptr = _gst_debug_category_new(
                name.to_glib_none().0,
                color.to_glib(),
                description.to_glib_none().0,
            );
            assert!(!ptr.is_null());
            DebugCategory(ptr::NonNull::new_unchecked(ptr))
        }
    }

    pub fn get(name: &str) -> Option<DebugCategory> {
        unsafe {
            extern "C" {
                fn _gst_debug_get_category(name: *const c_char) -> *mut ffi::GstDebugCategory;
            }

            let cat = _gst_debug_get_category(name.to_glib_none().0);

            if cat.is_null() {
                None
            } else {
                Some(DebugCategory(ptr::NonNull::new_unchecked(cat)))
            }
        }
    }

    pub fn get_threshold(self) -> ::DebugLevel {
        from_glib(unsafe { ffi::gst_debug_category_get_threshold(self.0.as_ptr()) })
    }

    pub fn set_threshold(self, threshold: ::DebugLevel) {
        unsafe { ffi::gst_debug_category_set_threshold(self.0.as_ptr(), threshold.to_glib()) }
    }

    pub fn reset_threshold(self) {
        unsafe { ffi::gst_debug_category_reset_threshold(self.0.as_ptr()) }
    }

    pub fn get_color(self) -> ::DebugColorFlags {
        unsafe { from_glib(ffi::gst_debug_category_get_color(self.0.as_ptr())) }
    }

    pub fn get_name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::gst_debug_category_get_name(self.0.as_ptr()))
                .to_str()
                .unwrap()
        }
    }

    pub fn get_description<'a>(self) -> Option<&'a str> {
        unsafe {
            let ptr = ffi::gst_debug_category_get_description(self.0.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[inline]
    pub fn log<O: IsA<::Object>>(
        self,
        obj: Option<&O>,
        level: ::DebugLevel,
        file: &str,
        module: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        unsafe {
            if level.to_glib() as i32 > self.0.as_ref().threshold {
                return;
            }
        }

        let obj_ptr = match obj {
            Some(obj) => obj.to_glib_none().0 as *mut gobject_ffi::GObject,
            None => ptr::null_mut(),
        };

        unsafe {
            ffi::gst_debug_log(
                self.0.as_ptr(),
                level.to_glib(),
                file.to_glib_none().0,
                module.to_glib_none().0,
                line as i32,
                obj_ptr,
                fmt::format(args).to_glib_none().0,
            );
        }
    }
}

unsafe impl Sync for DebugCategory {}
unsafe impl Send for DebugCategory {}

impl fmt::Debug for DebugCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugCategory")
            .field(&self.get_name())
            .finish()
    }
}

lazy_static! {
    pub static ref CAT_RUST: DebugCategory = DebugCategory::new(
        "GST_RUST",
        ::DebugColorFlags::UNDERLINE,
        "GStreamer's Rust binding core",
    );
}

macro_rules! declare_debug_category_from_name(
    ($cat:ident, $cat_name:expr) => (
        lazy_static! {
            pub static ref $cat: DebugCategory = DebugCategory::get($cat_name)
                .expect(&format!("Unable to find `DebugCategory` with name {}", $cat_name));
        }
    );
);

declare_debug_category_from_name!(CAT_DEFAULT, "default");
declare_debug_category_from_name!(CAT_GST_INIT, "GST_INIT");
declare_debug_category_from_name!(CAT_MEMORY, "GST_MEMORY");
declare_debug_category_from_name!(CAT_PARENTAGE, "GST_PARENTAGE");
declare_debug_category_from_name!(CAT_STATES, "GST_STATES");
declare_debug_category_from_name!(CAT_SCHEDULING, "GST_SCHEDULING");
declare_debug_category_from_name!(CAT_BUFFER, "GST_BUFFER");
declare_debug_category_from_name!(CAT_BUFFER_LIST, "GST_BUFFER_LIST");
declare_debug_category_from_name!(CAT_BUS, "GST_BUS");
declare_debug_category_from_name!(CAT_CAPS, "GST_CAPS");
declare_debug_category_from_name!(CAT_CLOCK, "GST_CLOCK");
declare_debug_category_from_name!(CAT_ELEMENT_PADS, "GST_ELEMENT_PADS");
declare_debug_category_from_name!(CAT_PADS, "GST_PADS");
declare_debug_category_from_name!(CAT_PERFORMANCE, "GST_PERFORMANCE");
declare_debug_category_from_name!(CAT_PIPELINE, "GST_PIPELINE");
declare_debug_category_from_name!(CAT_PLUGIN_LOADING, "GST_PLUGIN_LOADING");
declare_debug_category_from_name!(CAT_PLUGIN_INFO, "GST_PLUGIN_INFO");
declare_debug_category_from_name!(CAT_PROPERTIES, "GST_PROPERTIES");
declare_debug_category_from_name!(CAT_NEGOTIATION, "GST_NEGOTIATION");
declare_debug_category_from_name!(CAT_REFCOUNTING, "GST_REFCOUNTING");
declare_debug_category_from_name!(CAT_ERROR_SYSTEM, "GST_ERROR_SYSTEM");
declare_debug_category_from_name!(CAT_EVENT, "GST_EVENT");
declare_debug_category_from_name!(CAT_MESSAGE, "GST_MESSAGE");
declare_debug_category_from_name!(CAT_PARAMS, "GST_PARAMS");
declare_debug_category_from_name!(CAT_CALL_TRACE, "GST_CALL_TRACE");
declare_debug_category_from_name!(CAT_SIGNAL, "GST_SIGNAL");
declare_debug_category_from_name!(CAT_PROBE, "GST_PROBE");
declare_debug_category_from_name!(CAT_REGISTRY, "GST_REGISTRY");
declare_debug_category_from_name!(CAT_QOS, "GST_QOS");
declare_debug_category_from_name!(CAT_META, "GST_META");
declare_debug_category_from_name!(CAT_LOCKING, "GST_LOCKING");
declare_debug_category_from_name!(CAT_CONTEXT, "GST_CONTEXT");

#[macro_export]
macro_rules! gst_error(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_warning(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_fixme(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_info(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_debug(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_trace(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_memdump(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log_with_level(
    ($cat:expr, level: $level:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log($cat.clone(), Some($obj), $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
    ($cat:expr, level: $level:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log($cat.clone(), None as Option<&$crate::Object>, $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_existing() {
        ::init().unwrap();

        let perf_cat = DebugCategory::get("GST_PERFORMANCE")
            .expect("Unable to find `DebugCategory` with name \"GST_PERFORMANCE\"");
        assert_eq!(perf_cat.get_name(), CAT_PERFORMANCE.get_name());
    }

    #[test]
    fn new_and_log() {
        ::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat",
            ::DebugColorFlags::empty(),
            "some debug category",
        );

        gst_error!(cat, "meh");
        gst_warning!(cat, "meh");
        gst_fixme!(cat, "meh");
        gst_info!(cat, "meh");
        gst_debug!(cat, "meh");
        gst_log!(cat, "meh");
        gst_trace!(cat, "meh");
        gst_memdump!(cat, "meh");

        let obj = ::Bin::new("meh");
        gst_error!(cat, obj: &obj, "meh");
        gst_warning!(cat, obj: &obj, "meh");
        gst_fixme!(cat, obj: &obj, "meh");
        gst_info!(cat, obj: &obj, "meh");
        gst_debug!(cat, obj: &obj, "meh");
        gst_log!(cat, obj: &obj, "meh");
        gst_trace!(cat, obj: &obj, "meh");
        gst_memdump!(cat, obj: &obj, "meh");
    }
}
