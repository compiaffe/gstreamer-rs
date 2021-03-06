// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Clock;
use ClockType;
use Object;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct SystemClock(Object<ffi::GstSystemClock, ffi::GstSystemClockClass, SystemClockClass>) @extends Clock, Object;

    match fn {
        get_type => || ffi::gst_system_clock_get_type(),
    }
}

impl SystemClock {
    pub fn obtain() -> Clock {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_system_clock_obtain())
        }
    }

    pub fn set_default<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(new_clock: Q) {
        assert_initialized_main_thread!();
        let new_clock = new_clock.into();
        unsafe {
            ffi::gst_system_clock_set_default(new_clock.map(|p| p.as_ref()).to_glib_none().0);
        }
    }
}

unsafe impl Send for SystemClock {}
unsafe impl Sync for SystemClock {}

pub const NONE_SYSTEM_CLOCK: Option<&SystemClock> = None;

pub trait SystemClockExt: 'static {
    fn get_property_clock_type(&self) -> ClockType;

    fn set_property_clock_type(&self, clock_type: ClockType);

    fn connect_property_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SystemClock>> SystemClockExt for O {
    fn get_property_clock_type(&self) -> ClockType {
        unsafe {
            let mut value = Value::from_type(<ClockType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"clock-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_clock_type(&self, clock_type: ClockType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"clock-type\0".as_ptr() as *const _, Value::from(&clock_type).to_glib_none().0);
        }
    }

    fn connect_property_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::clock-type\0".as_ptr() as *const _,
                Some(transmute(notify_clock_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_clock_type_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstSystemClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SystemClock> {
    let f: &F = &*(f as *const F);
    f(&SystemClock::from_glib_borrow(this).unsafe_cast())
}
