// This file was generated by gir (a01311c+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct BufferFlags: u32 {
        const BUFFER_FLAG_LIVE = 16;
        const BUFFER_FLAG_DECODE_ONLY = 32;
        const BUFFER_FLAG_DISCONT = 64;
        const BUFFER_FLAG_RESYNC = 128;
        const BUFFER_FLAG_CORRUPTED = 256;
        const BUFFER_FLAG_MARKER = 512;
        const BUFFER_FLAG_HEADER = 1024;
        const BUFFER_FLAG_GAP = 2048;
        const BUFFER_FLAG_DROPPABLE = 4096;
        const BUFFER_FLAG_DELTA_UNIT = 8192;
        const BUFFER_FLAG_TAG_MEMORY = 16384;
        const BUFFER_FLAG_SYNC_AFTER = 32768;
        const BUFFER_FLAG_LAST = 1048576;
    }
}

#[doc(hidden)]
impl ToGlib for BufferFlags {
    type GlibType = ffi::GstBufferFlags;

    fn to_glib(&self) -> ffi::GstBufferFlags {
        ffi::GstBufferFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstBufferFlags> for BufferFlags {
    fn from_glib(value: ffi::GstBufferFlags) -> BufferFlags {
        skip_assert_initialized!();
        BufferFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for BufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_buffer_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstBufferFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for BufferFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct PadProbeType: u32 {
        const PAD_PROBE_TYPE_INVALID = 0;
        const PAD_PROBE_TYPE_IDLE = 1;
        const PAD_PROBE_TYPE_BLOCK = 2;
        const PAD_PROBE_TYPE_BUFFER = 16;
        const PAD_PROBE_TYPE_BUFFER_LIST = 32;
        const PAD_PROBE_TYPE_EVENT_DOWNSTREAM = 64;
        const PAD_PROBE_TYPE_EVENT_UPSTREAM = 128;
        const PAD_PROBE_TYPE_EVENT_FLUSH = 256;
        const PAD_PROBE_TYPE_QUERY_DOWNSTREAM = 512;
        const PAD_PROBE_TYPE_QUERY_UPSTREAM = 1024;
        const PAD_PROBE_TYPE_PUSH = 4096;
        const PAD_PROBE_TYPE_PULL = 8192;
        const PAD_PROBE_TYPE_BLOCKING = 3;
        const PAD_PROBE_TYPE_DATA_DOWNSTREAM = 112;
        const PAD_PROBE_TYPE_DATA_UPSTREAM = 128;
        const PAD_PROBE_TYPE_DATA_BOTH = 240;
        const PAD_PROBE_TYPE_BLOCK_DOWNSTREAM = 114;
        const PAD_PROBE_TYPE_BLOCK_UPSTREAM = 130;
        const PAD_PROBE_TYPE_EVENT_BOTH = 192;
        const PAD_PROBE_TYPE_QUERY_BOTH = 1536;
        const PAD_PROBE_TYPE_ALL_BOTH = 1776;
        const PAD_PROBE_TYPE_SCHEDULING = 12288;
    }
}

#[doc(hidden)]
impl ToGlib for PadProbeType {
    type GlibType = ffi::GstPadProbeType;

    fn to_glib(&self) -> ffi::GstPadProbeType {
        ffi::GstPadProbeType::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadProbeType> for PadProbeType {
    fn from_glib(value: ffi::GstPadProbeType) -> PadProbeType {
        skip_assert_initialized!();
        PadProbeType::from_bits_truncate(value.bits())
    }
}

impl StaticType for PadProbeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_pad_probe_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadProbeType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadProbeType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstPadProbeType::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for PadProbeType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SeekFlags: u32 {
        const SEEK_FLAG_NONE = 0;
        const SEEK_FLAG_FLUSH = 1;
        const SEEK_FLAG_ACCURATE = 2;
        const SEEK_FLAG_KEY_UNIT = 4;
        const SEEK_FLAG_SEGMENT = 8;
        const SEEK_FLAG_TRICKMODE = 16;
        const SEEK_FLAG_SKIP = 16;
        const SEEK_FLAG_SNAP_BEFORE = 32;
        const SEEK_FLAG_SNAP_AFTER = 64;
        const SEEK_FLAG_SNAP_NEAREST = 96;
        const SEEK_FLAG_TRICKMODE_KEY_UNITS = 128;
        const SEEK_FLAG_TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SeekFlags {
    type GlibType = ffi::GstSeekFlags;

    fn to_glib(&self) -> ffi::GstSeekFlags {
        ffi::GstSeekFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSeekFlags> for SeekFlags {
    fn from_glib(value: ffi::GstSeekFlags) -> SeekFlags {
        skip_assert_initialized!();
        SeekFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for SeekFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_seek_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SeekFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SeekFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstSeekFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SeekFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct StreamFlags: u32 {
        const STREAM_FLAG_NONE = 0;
        const STREAM_FLAG_SPARSE = 1;
        const STREAM_FLAG_SELECT = 2;
        const STREAM_FLAG_UNSELECT = 4;
    }
}

#[doc(hidden)]
impl ToGlib for StreamFlags {
    type GlibType = ffi::GstStreamFlags;

    fn to_glib(&self) -> ffi::GstStreamFlags {
        ffi::GstStreamFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamFlags> for StreamFlags {
    fn from_glib(value: ffi::GstStreamFlags) -> StreamFlags {
        skip_assert_initialized!();
        StreamFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for StreamFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_stream_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StreamFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StreamFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstStreamFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for StreamFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct StreamType: u32 {
        const STREAM_TYPE_UNKNOWN = 1;
        const STREAM_TYPE_AUDIO = 2;
        const STREAM_TYPE_VIDEO = 4;
        const STREAM_TYPE_CONTAINER = 8;
        const STREAM_TYPE_TEXT = 16;
    }
}

#[doc(hidden)]
impl ToGlib for StreamType {
    type GlibType = ffi::GstStreamType;

    fn to_glib(&self) -> ffi::GstStreamType {
        ffi::GstStreamType::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamType> for StreamType {
    fn from_glib(value: ffi::GstStreamType) -> StreamType {
        skip_assert_initialized!();
        StreamType::from_bits_truncate(value.bits())
    }
}

impl StaticType for StreamType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_stream_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StreamType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StreamType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstStreamType::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for StreamType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

