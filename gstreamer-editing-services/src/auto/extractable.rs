// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Extractable(Object<ffi::GESExtractable, ffi::GESExtractableInterface>);

    match fn {
        get_type => || ffi::ges_extractable_get_type(),
    }
}

pub trait ExtractableExt: 'static {
    fn get_asset(&self) -> Option<Asset>;

    fn get_id(&self) -> Option<GString>;

    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> bool;
}

impl<O: IsA<Extractable>> ExtractableExt for O {
    fn get_asset(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ffi::ges_extractable_get_asset(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::ges_extractable_get_id(self.to_glib_none().0))
        }
    }

    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> bool {
        unsafe {
            from_glib(ffi::ges_extractable_set_asset(self.to_glib_none().0, asset.to_glib_none().0))
        }
    }
}
