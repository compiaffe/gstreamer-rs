// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use gst;
use gst_base;

glib_wrapper! {
    pub struct VideoFilter(Object<ffi::GstVideoFilter, ffi::GstVideoFilterClass, VideoFilterClass>) @extends gst_base::BaseTransform, gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_video_filter_get_type(),
    }
}

impl VideoFilter {}

unsafe impl Send for VideoFilter {}
unsafe impl Sync for VideoFilter {}

pub const NONE_VIDEO_FILTER: Option<&VideoFilter> = None;
