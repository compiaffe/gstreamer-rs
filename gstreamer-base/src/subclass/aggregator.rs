// Copyright (C) 2017-2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

use ffi;
use glib_ffi;
use gst_ffi;

use glib::translate::*;
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use std::ptr;

use Aggregator;
use AggregatorClass;
use AggregatorPad;

pub trait AggregatorImpl: AggregatorImplExt + ElementImpl + Send + Sync + 'static {
    fn flush(&self, aggregator: &Aggregator) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_flush(aggregator)
    }

    fn clip(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        buffer: gst::Buffer,
    ) -> Option<gst::Buffer> {
        self.parent_clip(aggregator, aggregator_pad, buffer)
    }

    fn finish_buffer(
        &self,
        aggregator: &Aggregator,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_finish_buffer(aggregator, buffer)
    }

    fn sink_event(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        event: gst::Event,
    ) -> bool {
        self.parent_sink_event(aggregator, aggregator_pad, event)
    }

    fn sink_query(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        query: &mut gst::QueryRef,
    ) -> bool {
        self.parent_sink_query(aggregator, aggregator_pad, query)
    }

    fn src_event(&self, aggregator: &Aggregator, event: gst::Event) -> bool {
        self.parent_src_event(aggregator, event)
    }

    fn src_query(&self, aggregator: &Aggregator, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(aggregator, query)
    }

    fn src_activate(
        &self,
        aggregator: &Aggregator,
        mode: gst::PadMode,
        active: bool,
    ) -> Result<(), gst::LoggableError> {
        self.parent_src_activate(aggregator, mode, active)
    }

    fn aggregate(
        &self,
        aggregator: &Aggregator,
        timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn start(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage> {
        self.parent_start(aggregator)
    }

    fn stop(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(aggregator)
    }

    fn get_next_time(&self, aggregator: &Aggregator) -> gst::ClockTime {
        self.parent_get_next_time(aggregator)
    }

    fn create_new_pad(
        &self,
        aggregator: &Aggregator,
        templ: &gst::PadTemplate,
        req_name: Option<&str>,
        caps: Option<&gst::CapsRef>,
    ) -> Option<AggregatorPad> {
        self.parent_create_new_pad(aggregator, templ, req_name, caps)
    }

    fn update_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<gst::Caps, gst::FlowError> {
        self.parent_update_src_caps(aggregator, caps)
    }

    fn fixate_src_caps(&self, aggregator: &Aggregator, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate_src_caps(aggregator, caps)
    }

    fn negotiated_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_negotiated_src_caps(aggregator, caps)
    }
}

pub trait AggregatorImplExt {
    fn parent_flush(&self, aggregator: &Aggregator) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_clip(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        buffer: gst::Buffer,
    ) -> Option<gst::Buffer>;

    fn parent_finish_buffer(
        &self,
        aggregator: &Aggregator,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_sink_event(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        event: gst::Event,
    ) -> bool;

    fn parent_sink_query(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        query: &mut gst::QueryRef,
    ) -> bool;

    fn parent_src_event(&self, aggregator: &Aggregator, event: gst::Event) -> bool;

    fn parent_src_query(&self, aggregator: &Aggregator, query: &mut gst::QueryRef) -> bool;

    fn parent_src_activate(
        &self,
        aggregator: &Aggregator,
        mode: gst::PadMode,
        active: bool,
    ) -> Result<(), gst::LoggableError>;

    fn parent_aggregate(
        &self,
        aggregator: &Aggregator,
        timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_start(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage>;

    fn parent_get_next_time(&self, aggregator: &Aggregator) -> gst::ClockTime;

    fn parent_create_new_pad(
        &self,
        aggregator: &Aggregator,
        templ: &gst::PadTemplate,
        req_name: Option<&str>,
        caps: Option<&gst::CapsRef>,
    ) -> Option<AggregatorPad>;

    fn parent_update_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<gst::Caps, gst::FlowError>;

    fn parent_fixate_src_caps(&self, aggregator: &Aggregator, caps: gst::Caps) -> gst::Caps;

    fn parent_negotiated_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<(), gst::LoggableError>;
}

impl<T: AggregatorImpl + ObjectImpl> AggregatorImplExt for T {
    fn parent_flush(&self, aggregator: &Aggregator) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            (*parent_class)
                .flush
                .map(|f| from_glib(f(aggregator.to_glib_none().0)))
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_clip(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        buffer: gst::Buffer,
    ) -> Option<gst::Buffer> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            match (*parent_class).clip {
                None => Some(buffer),
                Some(ref func) => from_glib_full(func(
                    aggregator.to_glib_none().0,
                    aggregator_pad.to_glib_none().0,
                    buffer.into_ptr(),
                )),
            }
        }
    }

    fn parent_finish_buffer(
        &self,
        aggregator: &Aggregator,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .finish_buffer
                .expect("Missing parent function `finish_buffer`");
            gst::FlowReturn::from_glib(f(aggregator.to_glib_none().0, buffer.into_ptr()))
                .into_result()
        }
    }

    fn parent_sink_event(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        event: gst::Event,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                aggregator.to_glib_none().0,
                aggregator_pad.to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn parent_sink_query(
        &self,
        aggregator: &Aggregator,
        aggregator_pad: &AggregatorPad,
        query: &mut gst::QueryRef,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                aggregator.to_glib_none().0,
                aggregator_pad.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, aggregator: &Aggregator, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(aggregator.to_glib_none().0, event.into_ptr()))
        }
    }

    fn parent_src_query(&self, aggregator: &Aggregator, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(aggregator.to_glib_none().0, query.as_mut_ptr()))
        }
    }

    fn parent_src_activate(
        &self,
        aggregator: &Aggregator,
        mode: gst::PadMode,
        active: bool,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            match (*parent_class).src_activate {
                None => Ok(()),
                Some(f) => gst_result_from_gboolean!(
                    f(
                        aggregator.to_glib_none().0,
                        mode.to_glib(),
                        active.to_glib()
                    ),
                    gst::CAT_RUST,
                    "Parent function `src_activate` failed"
                ),
            }
        }
    }

    fn parent_aggregate(
        &self,
        aggregator: &Aggregator,
        timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .aggregate
                .expect("Missing parent function `aggregate`");
            gst::FlowReturn::from_glib(f(aggregator.to_glib_none().0, timeout.to_glib()))
                .into_result()
        }
    }

    fn parent_start(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(aggregator.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, aggregator: &Aggregator) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(aggregator.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_get_next_time(&self, aggregator: &Aggregator) -> gst::ClockTime {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            (*parent_class)
                .get_next_time
                .map(|f| from_glib(f(aggregator.to_glib_none().0)))
                .unwrap_or(gst::CLOCK_TIME_NONE)
        }
    }

    fn parent_create_new_pad(
        &self,
        aggregator: &Aggregator,
        templ: &gst::PadTemplate,
        req_name: Option<&str>,
        caps: Option<&gst::CapsRef>,
    ) -> Option<AggregatorPad> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .create_new_pad
                .expect("Missing parent function `create_new_pad`");
            from_glib_full(f(
                aggregator.to_glib_none().0,
                templ.to_glib_none().0,
                req_name.to_glib_none().0,
                caps.map(|c| c.as_ptr()).unwrap_or(ptr::null()),
            ))
        }
    }

    fn parent_update_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<gst::Caps, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            let f = (*parent_class)
                .update_src_caps
                .expect("Missing parent function `update_src_caps`");

            let mut out_caps = ptr::null_mut();
            gst::FlowReturn::from_glib(f(
                aggregator.to_glib_none().0,
                caps.as_mut_ptr(),
                &mut out_caps,
            ))
            .into_result_value(|| from_glib_full(out_caps))
        }
    }

    fn parent_fixate_src_caps(&self, aggregator: &Aggregator, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;

            let f = (*parent_class)
                .fixate_src_caps
                .expect("Missing parent function `fixate_src_caps`");
            from_glib_full(f(aggregator.to_glib_none().0, caps.into_ptr()))
        }
    }

    fn parent_negotiated_src_caps(
        &self,
        aggregator: &Aggregator,
        caps: &gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorClass;
            (*parent_class)
                .negotiated_src_caps
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(aggregator.to_glib_none().0, caps.as_mut_ptr()),
                        gst::CAT_RUST,
                        "Parent function `negotiated_src_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: ObjectSubclass + AggregatorImpl> IsSubclassable<T> for AggregatorClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut ffi::GstAggregatorClass);
            klass.flush = Some(aggregator_flush::<T>);
            klass.clip = Some(aggregator_clip::<T>);
            klass.finish_buffer = Some(aggregator_finish_buffer::<T>);
            klass.sink_event = Some(aggregator_sink_event::<T>);
            klass.sink_query = Some(aggregator_sink_query::<T>);
            klass.src_event = Some(aggregator_src_event::<T>);
            klass.src_query = Some(aggregator_src_query::<T>);
            klass.src_activate = Some(aggregator_src_activate::<T>);
            klass.aggregate = Some(aggregator_aggregate::<T>);
            klass.start = Some(aggregator_start::<T>);
            klass.stop = Some(aggregator_stop::<T>);
            klass.get_next_time = Some(aggregator_get_next_time::<T>);
            klass.create_new_pad = Some(aggregator_create_new_pad::<T>);
            klass.update_src_caps = Some(aggregator_update_src_caps::<T>);
            klass.fixate_src_caps = Some(aggregator_fixate_src_caps::<T>);
            klass.negotiated_src_caps = Some(aggregator_negotiated_src_caps::<T>);
        }
    }
}

unsafe extern "C" fn aggregator_flush<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
) -> gst_ffi::GstFlowReturn
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.flush(&wrap).into()
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_clip<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    aggregator_pad: *mut ffi::GstAggregatorPad,
    buffer: *mut gst_ffi::GstBuffer,
) -> *mut gst_ffi::GstBuffer
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    let ret = gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.clip(
            &wrap,
            &from_glib_borrow(aggregator_pad),
            from_glib_full(buffer),
        )
    });

    ret.map(|r| r.into_ptr()).unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn aggregator_finish_buffer<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    buffer: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.finish_buffer(&wrap, from_glib_full(buffer)).into()
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_sink_event<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    aggregator_pad: *mut ffi::GstAggregatorPad,
    event: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_event(
            &wrap,
            &from_glib_borrow(aggregator_pad),
            from_glib_full(event),
        )
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_sink_query<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    aggregator_pad: *mut ffi::GstAggregatorPad,
    query: *mut gst_ffi::GstQuery,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_query(
            &wrap,
            &from_glib_borrow(aggregator_pad),
            gst::QueryRef::from_mut_ptr(query),
        )
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_src_event<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    event: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_src_query<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    query: *mut gst_ffi::GstQuery,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_query(&wrap, gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_src_activate<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    mode: gst_ffi::GstPadMode,
    active: glib_ffi::gboolean,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.src_activate(&wrap, from_glib(mode), from_glib(active)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_aggregate<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    timeout: glib_ffi::gboolean,
) -> gst_ffi::GstFlowReturn
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.aggregate(&wrap, from_glib(timeout)).into()
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_start<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_get_next_time<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
) -> gst_ffi::GstClockTime
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::CLOCK_TIME_NONE, {
        imp.get_next_time(&wrap)
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_create_new_pad<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    templ: *mut gst_ffi::GstPadTemplate,
    req_name: *const libc::c_char,
    caps: *const gst_ffi::GstCaps,
) -> *mut ffi::GstAggregatorPad
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        let req_name: Option<String> = from_glib_none(req_name);

        // FIXME: Easier way to convert Option<String> to Option<&str>?
        let mut _tmp = String::new();
        let req_name = match req_name {
            Some(n) => {
                _tmp = n;
                Some(_tmp.as_str())
            }
            None => None,
        };

        imp.create_new_pad(
            &wrap,
            &from_glib_borrow(templ),
            req_name,
            if caps.is_null() {
                None
            } else {
                Some(gst::CapsRef::from_ptr(caps))
            },
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn aggregator_update_src_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    caps: *mut gst_ffi::GstCaps,
    res: *mut *mut gst_ffi::GstCaps,
) -> gst_ffi::GstFlowReturn
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    *res = ptr::null_mut();

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.update_src_caps(&wrap, gst::CapsRef::from_ptr(caps)) {
            Ok(res_caps) => {
                *res = res_caps.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => err.into(),
        }
    })
    .to_glib()
}

unsafe extern "C" fn aggregator_fixate_src_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    caps: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate_src_caps(&wrap, from_glib_full(caps))
    })
    .into_ptr()
}

unsafe extern "C" fn aggregator_negotiated_src_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstAggregator,
    caps: *mut gst_ffi::GstCaps,
) -> glib_ffi::gboolean
where
    T: AggregatorImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Aggregator = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.negotiated_src_caps(&wrap, gst::CapsRef::from_ptr(caps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&wrap);
                false
            }
        }
    })
    .to_glib()
}
