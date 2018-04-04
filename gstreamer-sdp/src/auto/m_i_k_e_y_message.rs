// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use MIKEYCacheType;
use MIKEYMapType;
use MIKEYPRFFunc;
use MIKEYPayload;
use MIKEYPayloadType;
use MIKEYType;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MIKEYMessage(Boxed<ffi::GstMIKEYMessage>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::gst_mikey_message_get_type(), ptr as *mut _) as *mut ffi::GstMIKEYMessage,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::gst_mikey_message_get_type(), ptr as *mut _),
        get_type => || ffi::gst_mikey_message_get_type(),
    }
}

impl MIKEYMessage {
    pub fn new() -> MIKEYMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_mikey_message_new())
        }
    }

    //pub fn new_from_bytes(bytes: &glib::Bytes, info: /*Ignored*/&mut MIKEYDecryptInfo, error: /*Ignored*/Option<Error>) -> MIKEYMessage {
    //    unsafe { TODO: call ffi::gst_mikey_message_new_from_bytes() }
    //}

    pub fn new_from_caps(caps: &gst::Caps) -> MIKEYMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_mikey_message_new_from_caps(caps.to_glib_none().0))
        }
    }

    //pub fn new_from_data(data: &[u8], info: /*Ignored*/&mut MIKEYDecryptInfo, error: /*Ignored*/Option<Error>) -> MIKEYMessage {
    //    unsafe { TODO: call ffi::gst_mikey_message_new_from_data() }
    //}

    pub fn add_cs_srtp(&mut self, policy: u8, ssrc: u32, roc: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_add_cs_srtp(self.to_glib_none_mut().0, policy, ssrc, roc))
        }
    }

    pub fn add_pke(&mut self, C: MIKEYCacheType, data: &[u8]) -> bool {
        let data_len = data.len() as u16;
        unsafe {
            from_glib(ffi::gst_mikey_message_add_pke(self.to_glib_none_mut().0, C.to_glib(), data_len, data.to_glib_none().0))
        }
    }

    pub fn add_rand(&mut self, rand: &[u8]) -> bool {
        let len = rand.len() as u8;
        unsafe {
            from_glib(ffi::gst_mikey_message_add_rand(self.to_glib_none_mut().0, len, rand.to_glib_none().0))
        }
    }

    pub fn add_rand_len(&mut self, len: u8) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_add_rand_len(self.to_glib_none_mut().0, len))
        }
    }

    //pub fn add_t(&mut self, type_: MIKEYTSType, ts_value: &[u8]) -> bool {
    //    unsafe { TODO: call ffi::gst_mikey_message_add_t() }
    //}

    pub fn add_t_now_ntp_utc(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_add_t_now_ntp_utc(self.to_glib_none_mut().0))
        }
    }

    pub fn find_payload(&self, type_: MIKEYPayloadType, nth: u32) -> Option<MIKEYPayload> {
        unsafe {
            from_glib_none(ffi::gst_mikey_message_find_payload(self.to_glib_none().0, type_.to_glib(), nth))
        }
    }

    //pub fn get_cs_srtp(&self, idx: u32) -> /*Ignored*/Option<MIKEYMapSRTP> {
    //    unsafe { TODO: call ffi::gst_mikey_message_get_cs_srtp() }
    //}

    pub fn get_n_cs(&self) -> u32 {
        unsafe {
            ffi::gst_mikey_message_get_n_cs(self.to_glib_none().0)
        }
    }

    pub fn get_n_payloads(&self) -> u32 {
        unsafe {
            ffi::gst_mikey_message_get_n_payloads(self.to_glib_none().0)
        }
    }

    //pub fn insert_cs_srtp(&mut self, idx: i32, map: /*Ignored*/&MIKEYMapSRTP) -> bool {
    //    unsafe { TODO: call ffi::gst_mikey_message_insert_cs_srtp() }
    //}

    pub fn remove_cs_srtp(&mut self, idx: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_remove_cs_srtp(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn remove_payload(&mut self, idx: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_remove_payload(self.to_glib_none_mut().0, idx))
        }
    }

    //pub fn replace_cs_srtp(&mut self, idx: i32, map: /*Ignored*/&MIKEYMapSRTP) -> bool {
    //    unsafe { TODO: call ffi::gst_mikey_message_replace_cs_srtp() }
    //}

    pub fn set_info(&mut self, version: u8, type_: MIKEYType, V: bool, prf_func: MIKEYPRFFunc, CSB_id: u32, map_type: MIKEYMapType) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_set_info(self.to_glib_none_mut().0, version, type_.to_glib(), V.to_glib(), prf_func.to_glib(), CSB_id, map_type.to_glib()))
        }
    }

    //pub fn to_bytes(&mut self, info: /*Ignored*/&mut MIKEYEncryptInfo, error: /*Ignored*/Option<Error>) -> Option<glib::Bytes> {
    //    unsafe { TODO: call ffi::gst_mikey_message_to_bytes() }
    //}

    #[cfg(any(feature = "v1_8_1", feature = "dox"))]
    pub fn to_caps(&self, caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_to_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }
}

impl Default for MIKEYMessage {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for MIKEYMessage {}
