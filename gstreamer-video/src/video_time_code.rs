// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::prelude::*;
use glib::translate::*;
use glib::value;
use glib_ffi;
use gobject_ffi;
use gst;
use gst::prelude::*;
use std::cmp;
use std::fmt;
use std::mem;
use std::ptr;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::str;

use VideoTimeCodeFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use VideoTimeCodeInterval;

pub struct VideoTimeCode(ffi::GstVideoTimeCode);
pub struct ValidVideoTimeCode(ffi::GstVideoTimeCode);

impl VideoTimeCode {
    pub fn new_empty() -> VideoTimeCode {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::zeroed();
            ffi::gst_video_time_code_clear(&mut v);
            VideoTimeCode(v)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fps: gst::Fraction,
        latest_daily_jam: Option<&glib::DateTime>,
        flags: VideoTimeCodeFlags,
        hours: u32,
        minutes: u32,
        seconds: u32,
        frames: u32,
        field_count: u32,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::zeroed();
            ffi::gst_video_time_code_init(
                &mut v,
                *fps.numer() as u32,
                *fps.denom() as u32,
                latest_daily_jam.to_glib_none().0,
                flags.to_glib(),
                hours,
                minutes,
                seconds,
                frames,
                field_count,
            );

            VideoTimeCode(v)
        }
    }

    //    #[cfg(any(feature = "v1_16", feature = "dox"))]
    //    pub fn new_from_date_time(
    //        fps: gst::Fraction,
    //        dt: &glib::DateTime,
    //        flags: VideoTimeCodeFlags,
    //        field_count: u32,
    //    ) -> Option<VideoTimeCode> {
    //        assert_initialized_main_thread!();
    //        assert!(fps_d > 0);
    //        unsafe {
    //            let mut v = mem::zeroed();
    //            let res = ffi::gst_video_time_code_init_from_date_time_full(
    //                &mut v,
    //                *fps.numer() as u32,
    //                *fps.denom() as u32,
    //                dt.to_glib_none().0,
    //                flags.to_glib(),
    //                field_count,
    //            );
    //
    //            if res == glib_ffi::GFALSE {
    //                None
    //            } else {
    //                Some(VideoTimeCode(v))
    //            }
    //        }
    //    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn from_string(tc_str: &str) -> Option<VideoTimeCode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_video_time_code_new_from_string(
                tc_str.to_glib_none().0,
            ))
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gst_video_time_code_is_valid(self.to_glib_none().0)) }
    }

    pub fn set_fps(&mut self, fps: gst::Fraction) {
        self.0.config.fps_n = *fps.numer() as u32;
        self.0.config.fps_d = *fps.denom() as u32;
    }

    pub fn set_flags(&mut self, flags: VideoTimeCodeFlags) {
        self.0.config.flags = flags.to_glib()
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.0.hours = hours
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        assert!(minutes < 60);
        self.0.minutes = minutes
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.0.seconds = seconds
    }

    pub fn set_frames(&mut self, frames: u32) {
        self.0.frames = frames
    }

    pub fn set_field_count(&mut self, field_count: u32) {
        assert!(field_count <= 2);
        self.0.field_count = field_count
    }

    pub fn try_into(self) -> Result<ValidVideoTimeCode, VideoTimeCode> {
        if self.is_valid() {
            Ok(ValidVideoTimeCode(self.0))
        } else {
            Err(self)
        }
    }
}

impl ValidVideoTimeCode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fps: gst::Fraction,
        latest_daily_jam: Option<&glib::DateTime>,
        flags: VideoTimeCodeFlags,
        hours: u32,
        minutes: u32,
        seconds: u32,
        frames: u32,
        field_count: u32,
    ) -> Option<Self> {
        let tc = VideoTimeCode::new(
            fps,
            latest_daily_jam,
            flags,
            hours,
            minutes,
            seconds,
            frames,
            field_count,
        );
        tc.try_into().ok()
    }

    //    #[cfg(any(feature = "v1_16", feature = "dox"))]
    //    pub fn new_from_date_time(
    //        fps: gst::Fraction,
    //        dt: &glib::DateTime,
    //        flags: VideoTimeCodeFlags,
    //        field_count: u32,
    //    ) -> Option<VideoTimeCode> {
    //        let tc = VideoTimeCode::new_from_date_time(fps, dt, flags, field_count);
    //        tc.and_then(|tc| tc.try_into().ok())
    //    }

    pub fn add_frames(&mut self, frames: i64) {
        unsafe {
            ffi::gst_video_time_code_add_frames(self.to_glib_none_mut().0, frames);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn add_interval(&self, tc_inter: &VideoTimeCodeInterval) -> Option<VideoTimeCode> {
        unsafe {
            from_glib_full(ffi::gst_video_time_code_add_interval(
                self.to_glib_none().0,
                tc_inter.to_glib_none().0,
            ))
        }
    }

    fn compare(&self, tc2: &ValidVideoTimeCode) -> i32 {
        unsafe { ffi::gst_video_time_code_compare(self.to_glib_none().0, tc2.to_glib_none().0) }
    }

    pub fn frames_since_daily_jam(&self) -> u64 {
        unsafe { ffi::gst_video_time_code_frames_since_daily_jam(self.to_glib_none().0) }
    }

    pub fn increment_frame(&mut self) {
        unsafe {
            ffi::gst_video_time_code_increment_frame(self.to_glib_none_mut().0);
        }
    }

    pub fn nsec_since_daily_jam(&self) -> u64 {
        unsafe { ffi::gst_video_time_code_nsec_since_daily_jam(self.to_glib_none().0) }
    }

    pub fn to_date_time(&self) -> Option<glib::DateTime> {
        unsafe { from_glib_full(ffi::gst_video_time_code_to_date_time(self.to_glib_none().0)) }
    }
}

macro_rules! generic_impl {
    ($name:ident) => {
        impl $name {
            pub fn to_string(&self) -> String {
                unsafe { from_glib_full(ffi::gst_video_time_code_to_string(self.to_glib_none().0)) }
            }

            pub fn get_hours(&self) -> u32 {
                self.0.hours
            }

            pub fn get_minutes(&self) -> u32 {
                self.0.minutes
            }

            pub fn get_seconds(&self) -> u32 {
                self.0.seconds
            }

            pub fn get_frames(&self) -> u32 {
                self.0.frames
            }

            pub fn get_field_count(&self) -> u32 {
                self.0.field_count
            }

            pub fn get_fps(&self) -> gst::Fraction {
                (self.0.config.fps_n as i32, self.0.config.fps_d as i32).into()
            }

            pub fn get_flags(&self) -> VideoTimeCodeFlags {
                from_glib(self.0.config.flags)
            }

            pub fn get_latest_daily_jam(&self) -> Option<glib::DateTime> {
                unsafe { from_glib_none(self.0.config.latest_daily_jam) }
            }

            pub fn set_latest_daily_jam(&mut self, latest_daily_jam: Option<&glib::DateTime>) {
                unsafe {
                    if !self.0.config.latest_daily_jam.is_null() {
                        glib_ffi::g_date_time_unref(self.0.config.latest_daily_jam);
                    }

                    self.0.config.latest_daily_jam = latest_daily_jam.to_glib_full()
                }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe {
                    let v = self.0;
                    if !v.config.latest_daily_jam.is_null() {
                        glib_ffi::g_date_time_ref(v.config.latest_daily_jam);
                    }

                    $name(v)
                }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    if !self.0.config.latest_daily_jam.is_null() {
                        glib_ffi::g_date_time_unref(self.0.config.latest_daily_jam);
                    }
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                f.debug_struct("$name")
                    .field("fps", &self.get_fps())
                    .field("flags", &self.get_flags())
                    .field("latest_daily_jam", &self.get_latest_daily_jam())
                    .field("hours", &self.get_hours())
                    .field("minutes", &self.get_minutes())
                    .field("seconds", &self.get_seconds())
                    .field("frames", &self.get_frames())
                    .field("field_count", &self.get_field_count())
                    .finish()
            }
        }

        impl fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        #[doc(hidden)]
        impl GlibPtrDefault for $name {
            type GlibType = *mut ffi::GstVideoTimeCode;
        }

        #[doc(hidden)]
        impl<'a> ToGlibPtr<'a, *const ffi::GstVideoTimeCode> for $name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstVideoTimeCode, Self> {
                Stash(&self.0 as *const _, self)
            }

            #[inline]
            fn to_glib_full(&self) -> *const ffi::GstVideoTimeCode {
                unsafe { ffi::gst_video_time_code_copy(&self.0 as *const _) }
            }
        }

        #[doc(hidden)]
        impl<'a> ToGlibPtrMut<'a, *mut ffi::GstVideoTimeCode> for $name {
            type Storage = &'a mut Self;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GstVideoTimeCode, Self> {
                let ptr = &mut self.0 as *mut _;
                StashMut(ptr, self)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrNone<*mut ffi::GstVideoTimeCode> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut ffi::GstVideoTimeCode) -> Self {
                assert!(!ptr.is_null());
                let v = ptr::read(ptr);
                if !v.config.latest_daily_jam.is_null() {
                    glib_ffi::g_date_time_ref(v.config.latest_daily_jam);
                }

                $name(v)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrNone<*const ffi::GstVideoTimeCode> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *const ffi::GstVideoTimeCode) -> Self {
                assert!(!ptr.is_null());
                let v = ptr::read(ptr);
                if !v.config.latest_daily_jam.is_null() {
                    glib_ffi::g_date_time_ref(v.config.latest_daily_jam);
                }

                $name(v)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::GstVideoTimeCode> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ffi::GstVideoTimeCode) -> Self {
                assert!(!ptr.is_null());
                let v = ptr::read(ptr);
                if !v.config.latest_daily_jam.is_null() {
                    glib_ffi::g_date_time_ref(v.config.latest_daily_jam);
                }
                ffi::gst_video_time_code_free(ptr);

                $name(v)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrBorrow<*mut ffi::GstVideoTimeCode> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut ffi::GstVideoTimeCode) -> Self {
                assert!(!ptr.is_null());
                let v = ptr::read(ptr);
                if !v.config.latest_daily_jam.is_null() {
                    glib_ffi::g_date_time_ref(v.config.latest_daily_jam);
                }

                $name(v)
            }
        }

        impl StaticType for $name {
            fn static_type() -> glib::Type {
                unsafe { from_glib(ffi::gst_video_time_code_get_type()) }
            }
        }

        #[doc(hidden)]
        impl<'a> value::FromValueOptional<'a> for $name {
            unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
                Option::<$name>::from_glib_none(gobject_ffi::g_value_get_boxed(
                    value.to_glib_none().0,
                ) as *mut ffi::GstVideoTimeCode)
            }
        }

        #[doc(hidden)]
        impl value::SetValue for $name {
            unsafe fn set_value(value: &mut glib::Value, this: &Self) {
                gobject_ffi::g_value_set_boxed(
                    value.to_glib_none_mut().0,
                    ToGlibPtr::<*const ffi::GstVideoTimeCode>::to_glib_none(this).0
                        as glib_ffi::gpointer,
                )
            }
        }

        #[doc(hidden)]
        impl value::SetValueOptional for $name {
            unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
                gobject_ffi::g_value_set_boxed(
                    value.to_glib_none_mut().0,
                    ToGlibPtr::<*const ffi::GstVideoTimeCode>::to_glib_none(&this).0
                        as glib_ffi::gpointer,
                )
            }
        }
    };
}

generic_impl!(VideoTimeCode);
generic_impl!(ValidVideoTimeCode);

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl str::FromStr for VideoTimeCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();
        Self::from_string(s).ok_or(())
    }
}

impl PartialEq for ValidVideoTimeCode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for ValidVideoTimeCode {}

impl PartialOrd for ValidVideoTimeCode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for ValidVideoTimeCode {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl From<ValidVideoTimeCode> for VideoTimeCode {
    fn from(v: ValidVideoTimeCode) -> VideoTimeCode {
        VideoTimeCode(v.0)
    }
}

#[repr(C)]
pub struct VideoTimeCodeMeta(ffi::GstVideoTimeCodeMeta);

impl VideoTimeCodeMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        tc: &ValidVideoTimeCode,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        unsafe {
            let meta = ffi::gst_buffer_add_video_time_code_meta(
                buffer.as_mut_ptr(),
                tc.to_glib_none().0 as *mut _,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_tc(&self) -> ValidVideoTimeCode {
        unsafe { ValidVideoTimeCode::from_glib_none(&self.0.tc as *const _) }
    }

    pub fn set_tc(&mut self, tc: ValidVideoTimeCode) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            ffi::gst_video_time_code_clear(&mut self.0.tc);
            self.0.tc = tc.0;
            if !self.0.tc.config.latest_daily_jam.is_null() {
                glib_ffi::g_date_time_ref(self.0.tc.config.latest_daily_jam);
            }
        }
    }
}

unsafe impl MetaAPI for VideoTimeCodeMeta {
    type GstType = ffi::GstVideoTimeCodeMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoTimeCodeMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoTimeCodeMeta")
            .field("tc", &self.get_tc())
            .finish()
    }
}
