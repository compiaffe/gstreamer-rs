// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std;
use std::ptr;

glib_wrapper! {
    pub struct Preset(Object<ffi::GstPreset, ffi::GstPresetInterface>);

    match fn {
        get_type => || ffi::gst_preset_get_type(),
    }
}

impl Preset {
    pub fn get_app_dir() -> Option<std::path::PathBuf> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_preset_get_app_dir())
        }
    }

    pub fn set_app_dir<P: AsRef<std::path::Path>>(app_dir: P) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_set_app_dir(app_dir.as_ref().to_glib_none().0), "Failed to set app preset directory")
        }
    }
}

unsafe impl Send for Preset {}
unsafe impl Sync for Preset {}

pub trait PresetExt: 'static {
    fn delete_preset(&self, name: &str) -> Result<(), glib::error::BoolError>;

    fn get_meta(&self, name: &str, tag: &str) -> Option<GString>;

    fn get_preset_names(&self) -> Vec<GString>;

    fn get_property_names(&self) -> Vec<GString>;

    fn is_editable(&self) -> bool;

    fn load_preset(&self, name: &str) -> Result<(), glib::error::BoolError>;

    fn rename_preset(&self, old_name: &str, new_name: &str) -> Result<(), glib::error::BoolError>;

    fn save_preset(&self, name: &str) -> Result<(), glib::error::BoolError>;

    fn set_meta<'a, P: Into<Option<&'a str>>>(&self, name: &str, tag: &str, value: P) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<Preset>> PresetExt for O {
    fn delete_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_delete_preset(self.to_glib_none().0, name.to_glib_none().0), "Failed to delete preset")
        }
    }

    fn get_meta(&self, name: &str, tag: &str) -> Option<GString> {
        unsafe {
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::gst_preset_get_meta(self.to_glib_none().0, name.to_glib_none().0, tag.to_glib_none().0, &mut value));
            if ret { Some(from_glib_full(value)) } else { None }
        }
    }

    fn get_preset_names(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_preset_names(self.to_glib_none().0))
        }
    }

    fn get_property_names(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_property_names(self.to_glib_none().0))
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_is_editable(self.to_glib_none().0))
        }
    }

    fn load_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_load_preset(self.to_glib_none().0, name.to_glib_none().0), "Failed to load preset")
        }
    }

    fn rename_preset(&self, old_name: &str, new_name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_rename_preset(self.to_glib_none().0, old_name.to_glib_none().0, new_name.to_glib_none().0), "Failed to rename preset")
        }
    }

    fn save_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_save_preset(self.to_glib_none().0, name.to_glib_none().0), "Failed to save preset")
        }
    }

    fn set_meta<'a, P: Into<Option<&'a str>>>(&self, name: &str, tag: &str, value: P) -> Result<(), glib::error::BoolError> {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_preset_set_meta(self.to_glib_none().0, name.to_glib_none().0, tag.to_glib_none().0, value.0), "Failed to set preset meta")
        }
    }
}
