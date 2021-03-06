// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_sys;
use gst_video_sys;

use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};
use gst;

use std::mem;
use std::ptr;

pub fn convert_sample(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
) -> Result<gst::Sample, glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gst_video_sys::gst_video_convert_sample(
            sample.to_glib_none().0,
            caps.to_glib_none().0,
            timeout.to_glib(),
            &mut error,
        );

        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn convert_sample_async<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
{
    skip_assert_initialized!();
    unsafe { convert_sample_async_unsafe(sample, caps, timeout, func) }
}

pub fn convert_sample_async_local<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
{
    skip_assert_initialized!();
    unsafe {
        assert!(glib::MainContext::ref_thread_default().is_owner());
        convert_sample_async_unsafe(sample, caps, timeout, func)
    }
}

unsafe fn convert_sample_async_unsafe<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
{
    unsafe extern "C" fn convert_sample_async_trampoline<F>(
        sample: *mut gst_sys::GstSample,
        error: *mut glib_sys::GError,
        user_data: glib_sys::gpointer,
    ) where
        F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
    {
        let callback: &mut Option<F> = &mut *(user_data as *mut Option<F>);
        let callback = callback.take().unwrap();

        if error.is_null() {
            callback(Ok(from_glib_full(sample)))
        } else {
            callback(Err(from_glib_full(error)))
        }
    }
    unsafe extern "C" fn convert_sample_async_free<F>(user_data: glib_sys::gpointer)
    where
        F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
    {
        let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
    }

    let user_data: Box<Option<F>> = Box::new(Some(func));

    gst_video_sys::gst_video_convert_sample_async(
        sample.to_glib_none().0,
        caps.to_glib_none().0,
        timeout.to_glib(),
        Some(convert_sample_async_trampoline::<F>),
        Box::into_raw(user_data) as glib_sys::gpointer,
        Some(convert_sample_async_free::<F>),
    );
}

pub fn convert_sample_future(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<gst::Sample, glib::Error>> + 'static>>
{
    skip_assert_initialized!();

    use futures_channel::oneshot;
    use futures_util::future::lazy;
    use futures_util::future::FutureExt;

    let (sender, receiver) = oneshot::channel();

    let sample = sample.clone();
    let caps = caps.clone();
    let future = lazy(move |_| {
        assert!(
            glib::MainContext::ref_thread_default().is_owner(),
            "Spawning futures only allowed if the thread is owning the MainContext"
        );

        convert_sample_async(&sample, &caps, timeout, move |res| {
            let _ = sender.send(res);
        });
    })
    .then(|_| receiver.map(|res| res.expect("Sender dropped before callback was called")));

    Box::pin(future)
}

pub fn calculate_display_ratio(
    video_width: u32,
    video_height: u32,
    video_par: gst::Fraction,
    display_par: gst::Fraction,
) -> Option<gst::Fraction> {
    skip_assert_initialized!();

    unsafe {
        let mut dar_n = mem::MaybeUninit::uninit();
        let mut dar_d = mem::MaybeUninit::uninit();

        let res: bool = from_glib(gst_video_sys::gst_video_calculate_display_ratio(
            dar_n.as_mut_ptr(),
            dar_d.as_mut_ptr(),
            video_width,
            video_height,
            *video_par.numer() as u32,
            *video_par.denom() as u32,
            *display_par.numer() as u32,
            *display_par.denom() as u32,
        ));
        if res {
            Some(gst::Fraction::new(
                dar_n.assume_init() as i32,
                dar_d.assume_init() as i32,
            ))
        } else {
            None
        }
    }
}

pub fn guess_framerate(duration: gst::ClockTime) -> Option<gst::Fraction> {
    skip_assert_initialized!();

    unsafe {
        let mut dest_n = mem::MaybeUninit::uninit();
        let mut dest_d = mem::MaybeUninit::uninit();
        let res: bool = from_glib(gst_video_sys::gst_video_guess_framerate(
            duration.to_glib(),
            dest_n.as_mut_ptr(),
            dest_d.as_mut_ptr(),
        ));
        if res {
            Some(gst::Fraction::new(
                dest_n.assume_init() as i32,
                dest_d.assume_init() as i32,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib;
    use gst;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_convert_sample_async() {
        gst::init().unwrap();

        let l = glib::MainLoop::new(None, false);

        let mut in_buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let buffer = in_buffer.get_mut().unwrap();
            let mut data = buffer.map_writable().unwrap();

            for p in data.as_mut_slice().chunks_mut(4) {
                p[0] = 63;
                p[1] = 127;
                p[2] = 191;
                p[3] = 255;
            }
        }
        let in_caps = ::VideoInfo::new(::VideoFormat::Rgba, 320, 240)
            .build()
            .unwrap()
            .to_caps()
            .unwrap();
        let sample = gst::Sample::new().buffer(&in_buffer).caps(&in_caps).build();

        let out_caps = ::VideoInfo::new(::VideoFormat::Abgr, 320, 240)
            .build()
            .unwrap()
            .to_caps()
            .unwrap();

        let l_clone = l.clone();
        let res_store = Arc::new(Mutex::new(None));
        let res_store_clone = res_store.clone();
        convert_sample_async(&sample, &out_caps, gst::CLOCK_TIME_NONE, move |res| {
            *res_store_clone.lock().unwrap() = Some(res);
            l_clone.quit();
        });

        l.run();

        let res = res_store.lock().unwrap().take().unwrap();
        let res = res.unwrap();

        let converted_out_caps = res.get_caps().unwrap();
        assert_eq!(out_caps.as_ref(), converted_out_caps);
        let out_buffer = res.get_buffer().unwrap();
        {
            let data = out_buffer.map_readable().unwrap();

            for p in data.as_slice().chunks(4) {
                assert_eq!(p, &[255, 191, 127, 63]);
            }
        }
    }
}
