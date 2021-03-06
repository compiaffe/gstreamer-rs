// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use Error;
use Extractable;
use Group;
use Layer;
use TimelineElement;
use Track;
use TrackElement;
use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gst;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Timeline(Object<ffi::GESTimeline, ffi::GESTimelineClass, TimelineClass>) @extends gst::Element, gst::Object, @implements Extractable;

    match fn {
        get_type => || ffi::ges_timeline_get_type(),
    }
}

impl Timeline {
    pub fn new() -> Timeline {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_timeline_new())
        }
    }

    pub fn new_audio_video() -> Timeline {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_timeline_new_audio_video())
        }
    }

    pub fn new_from_uri(uri: &str) -> Result<Option<Timeline>, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_timeline_new_from_uri(uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TIMELINE: Option<&Timeline> = None;

pub trait TimelineExt: 'static {
    fn add_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    fn add_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError>;

    fn append_layer(&self) -> Layer;

    fn commit(&self) -> bool;

    fn commit_sync(&self) -> bool;

    fn get_auto_transition(&self) -> bool;

    fn get_duration(&self) -> gst::ClockTime;

    fn get_element(&self, name: &str) -> Option<TimelineElement>;

    fn get_groups(&self) -> Vec<Group>;

    fn get_layer(&self, priority: u32) -> Option<Layer>;

    fn get_layers(&self) -> Vec<Layer>;

    fn get_pad_for_track<P: IsA<Track>>(&self, track: &P) -> Option<gst::Pad>;

    fn get_snapping_distance(&self) -> gst::ClockTime;

    fn get_track_for_pad<P: IsA<gst::Pad>>(&self, pad: &P) -> Option<Track>;

    fn get_tracks(&self) -> Vec<Track>;

    fn is_empty(&self) -> bool;

    fn load_from_uri(&self, uri: &str) -> Result<(), Error>;

    fn move_layer<P: IsA<Layer>>(&self, layer: &P, new_layer_priority: u32) -> Result<(), glib::error::BoolError>;

    fn paste_element<P: IsA<TimelineElement>>(&self, element: &P, position: gst::ClockTime, layer_priority: i32) -> Option<TimelineElement>;

    fn remove_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    fn remove_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError>;

    fn save_to_uri<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, uri: &str, formatter_asset: Q, overwrite: bool) -> Result<(), Error>;

    fn set_auto_transition(&self, auto_transition: bool);

    fn set_snapping_distance(&self, snapping_distance: gst::ClockTime);

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_group_added<F: Fn(&Self, &Group) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_group_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_layer_added<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_layer_removed<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_select_tracks_for_object<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_snapping_ended<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_snapping_started<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_added<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_removed<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_snapping_distance_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Timeline>> TimelineExt for O {
    fn add_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::ges_timeline_add_layer(self.as_ref().to_glib_none().0, layer.as_ref().to_glib_none().0), "Failed to add layer")
        }
    }

    fn add_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::ges_timeline_add_track(self.as_ref().to_glib_none().0, track.as_ref().to_glib_full()), "Failed to add track")
        }
    }

    fn append_layer(&self) -> Layer {
        unsafe {
            from_glib_none(ffi::ges_timeline_append_layer(self.as_ref().to_glib_none().0))
        }
    }

    fn commit(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_commit(self.as_ref().to_glib_none().0))
        }
    }

    fn commit_sync(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_commit_sync(self.as_ref().to_glib_none().0))
        }
    }

    fn get_auto_transition(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_get_auto_transition(self.as_ref().to_glib_none().0))
        }
    }

    fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_get_duration(self.as_ref().to_glib_none().0))
        }
    }

    fn get_element(&self, name: &str) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_get_element(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_groups(&self) -> Vec<Group> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_timeline_get_groups(self.as_ref().to_glib_none().0))
        }
    }

    fn get_layer(&self, priority: u32) -> Option<Layer> {
        unsafe {
            from_glib_full(ffi::ges_timeline_get_layer(self.as_ref().to_glib_none().0, priority))
        }
    }

    fn get_layers(&self) -> Vec<Layer> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_timeline_get_layers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_pad_for_track<P: IsA<Track>>(&self, track: &P) -> Option<gst::Pad> {
        unsafe {
            from_glib_none(ffi::ges_timeline_get_pad_for_track(self.as_ref().to_glib_none().0, track.as_ref().to_glib_none().0))
        }
    }

    fn get_snapping_distance(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_get_snapping_distance(self.as_ref().to_glib_none().0))
        }
    }

    fn get_track_for_pad<P: IsA<gst::Pad>>(&self, pad: &P) -> Option<Track> {
        unsafe {
            from_glib_none(ffi::ges_timeline_get_track_for_pad(self.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0))
        }
    }

    fn get_tracks(&self) -> Vec<Track> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_timeline_get_tracks(self.as_ref().to_glib_none().0))
        }
    }

    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_is_empty(self.as_ref().to_glib_none().0))
        }
    }

    fn load_from_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_load_from_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn move_layer<P: IsA<Layer>>(&self, layer: &P, new_layer_priority: u32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::ges_timeline_move_layer(self.as_ref().to_glib_none().0, layer.as_ref().to_glib_none().0, new_layer_priority), "Failed to move layer")
        }
    }

    fn paste_element<P: IsA<TimelineElement>>(&self, element: &P, position: gst::ClockTime, layer_priority: i32) -> Option<TimelineElement> {
        unsafe {
            from_glib_none(ffi::ges_timeline_paste_element(self.as_ref().to_glib_none().0, element.as_ref().to_glib_none().0, position.to_glib(), layer_priority))
        }
    }

    fn remove_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::ges_timeline_remove_layer(self.as_ref().to_glib_none().0, layer.as_ref().to_glib_none().0), "Failed to remove layer")
        }
    }

    fn remove_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::ges_timeline_remove_track(self.as_ref().to_glib_none().0, track.as_ref().to_glib_none().0), "Failed to remove track")
        }
    }

    fn save_to_uri<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, uri: &str, formatter_asset: Q, overwrite: bool) -> Result<(), Error> {
        let formatter_asset = formatter_asset.into();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_save_to_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0, formatter_asset.map(|p| p.as_ref()).to_glib_none().0, overwrite.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_auto_transition(&self, auto_transition: bool) {
        unsafe {
            ffi::ges_timeline_set_auto_transition(self.as_ref().to_glib_none().0, auto_transition.to_glib());
        }
    }

    fn set_snapping_distance(&self, snapping_distance: gst::ClockTime) {
        unsafe {
            ffi::ges_timeline_set_snapping_distance(self.as_ref().to_glib_none().0, snapping_distance.to_glib());
        }
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"commited\0".as_ptr() as *const _,
                Some(transmute(commited_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_group_added<F: Fn(&Self, &Group) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"group-added\0".as_ptr() as *const _,
                Some(transmute(group_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_group_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype children: *.PtrArray TypeId { ns_id: 1, id: 51 }
    //}

    fn connect_layer_added<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"layer-added\0".as_ptr() as *const _,
                Some(transmute(layer_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_layer_removed<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"layer-removed\0".as_ptr() as *const _,
                Some(transmute(layer_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_select_tracks_for_object<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype return value *.PtrArray TypeId { ns_id: 1, id: 16 }
    //}

    fn connect_snapping_ended<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"snapping-ended\0".as_ptr() as *const _,
                Some(transmute(snapping_ended_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_snapping_started<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"snapping-started\0".as_ptr() as *const _,
                Some(transmute(snapping_started_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_track_added<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"track-added\0".as_ptr() as *const _,
                Some(transmute(track_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_track_removed<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"track-removed\0".as_ptr() as *const _,
                Some(transmute(track_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::auto-transition\0".as_ptr() as *const _,
                Some(transmute(notify_auto_transition_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::duration\0".as_ptr() as *const _,
                Some(transmute(notify_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_snapping_distance_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::snapping-distance\0".as_ptr() as *const _,
                Some(transmute(notify_snapping_distance_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn commited_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GESTimeline, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn group_added_trampoline<P, F: Fn(&P, &Group) + 'static>(this: *mut ffi::GESTimeline, group: *mut ffi::GESGroup, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(group))
}

unsafe extern "C" fn layer_added_trampoline<P, F: Fn(&P, &Layer) + 'static>(this: *mut ffi::GESTimeline, layer: *mut ffi::GESLayer, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(layer))
}

unsafe extern "C" fn layer_removed_trampoline<P, F: Fn(&P, &Layer) + 'static>(this: *mut ffi::GESTimeline, layer: *mut ffi::GESLayer, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(layer))
}

unsafe extern "C" fn snapping_ended_trampoline<P, F: Fn(&P, &TrackElement, &TrackElement, u64) + 'static>(this: *mut ffi::GESTimeline, object: *mut ffi::GESTrackElement, p0: *mut ffi::GESTrackElement, p1: u64, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object), &from_glib_borrow(p0), p1)
}

unsafe extern "C" fn snapping_started_trampoline<P, F: Fn(&P, &TrackElement, &TrackElement, u64) + 'static>(this: *mut ffi::GESTimeline, object: *mut ffi::GESTrackElement, p0: *mut ffi::GESTrackElement, p1: u64, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object), &from_glib_borrow(p0), p1)
}

unsafe extern "C" fn track_added_trampoline<P, F: Fn(&P, &Track) + 'static>(this: *mut ffi::GESTimeline, track: *mut ffi::GESTrack, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(track))
}

unsafe extern "C" fn track_removed_trampoline<P, F: Fn(&P, &Track) + 'static>(this: *mut ffi::GESTimeline, track: *mut ffi::GESTrack, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(track))
}

unsafe extern "C" fn notify_auto_transition_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GESTimeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GESTimeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_snapping_distance_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GESTimeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Timeline> {
    let f: &F = &*(f as *const F);
    f(&Timeline::from_glib_borrow(this).unsafe_cast())
}
