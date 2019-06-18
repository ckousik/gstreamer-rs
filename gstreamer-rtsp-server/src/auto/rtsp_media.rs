// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RTSPAddressPool;
use RTSPMediaStatus;
use RTSPPublishClockMode;
use RTSPStream;
use RTSPSuspendMode;
use RTSPTransportMode;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst;
use gst_rtsp;
use gst_rtsp_server_sys;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RTSPMedia(Object<gst_rtsp_server_sys::GstRTSPMedia, gst_rtsp_server_sys::GstRTSPMediaClass, RTSPMediaClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_media_get_type(),
    }
}

impl RTSPMedia {
    pub fn new<P: IsA<gst::Element>>(element: &P) -> RTSPMedia {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_new(element.as_ref().to_glib_full()))
        }
    }
}

unsafe impl Send for RTSPMedia {}
unsafe impl Sync for RTSPMedia {}

pub const NONE_RTSP_MEDIA: Option<&RTSPMedia> = None;

pub trait RTSPMediaExt: 'static {
    fn collect_streams(&self);

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn complete_pipeline(&self, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 10, id: 31 }) -> bool;

    fn create_stream<P: IsA<gst::Element>, Q: IsA<gst::Pad>>(&self, payloader: &P, pad: &Q) -> Option<RTSPStream>;

    fn find_stream(&self, control: &str) -> Option<RTSPStream>;

    fn get_address_pool(&self) -> Option<RTSPAddressPool>;

    fn get_base_time(&self) -> gst::ClockTime;

    fn get_buffer_size(&self) -> u32;

    fn get_clock(&self) -> Option<gst::Clock>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_do_retransmission(&self) -> bool;

    fn get_element(&self) -> Option<gst::Element>;

    fn get_latency(&self) -> u32;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_max_mcast_ttl(&self) -> u32;

    fn get_multicast_iface(&self) -> Option<GString>;

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions>;

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile;

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans;

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode;

    fn get_range_string(&self, play: bool, unit: gst_rtsp::RTSPRangeUnit) -> Option<GString>;

    fn get_retransmission_time(&self) -> gst::ClockTime;

    fn get_status(&self) -> RTSPMediaStatus;

    fn get_stream(&self, idx: u32) -> Option<RTSPStream>;

    fn get_suspend_mode(&self) -> RTSPSuspendMode;

    //fn get_time_provider(&self, address: Option<&str>, port: u16) -> /*Ignored*/Option<gst_net::NetTimeProvider>;

    fn get_transport_mode(&self) -> RTSPTransportMode;

    //fn handle_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn is_bind_mcast_address(&self) -> bool;

    fn is_eos_shutdown(&self) -> bool;

    fn is_reusable(&self) -> bool;

    fn is_shared(&self) -> bool;

    fn is_stop_on_disconnect(&self) -> bool;

    fn is_time_provider(&self) -> bool;

    fn n_streams(&self) -> u32;

    //fn prepare(&self, thread: /*Ignored*/Option<&mut RTSPThread>) -> bool;

    //fn seek(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange) -> bool;

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn seek_full(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange, flags: /*Ignored*/gst::SeekFlags) -> bool;

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn seekable(&self) -> /*Ignored*/gst::ClockTimeDiff;

    fn set_address_pool<P: IsA<RTSPAddressPool>>(&self, pool: Option<&P>);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool);

    fn set_buffer_size(&self, size: u32);

    fn set_clock<P: IsA<gst::Clock>>(&self, clock: Option<&P>);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_do_retransmission(&self, do_retransmission: bool);

    fn set_eos_shutdown(&self, eos_shutdown: bool);

    fn set_latency(&self, latency: u32);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool;

    fn set_multicast_iface(&self, multicast_iface: Option<&str>);

    //fn set_permissions(&self, permissions: /*Ignored*/Option<&mut RTSPPermissions>);

    fn set_pipeline_state(&self, state: gst::State);

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile);

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans);

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode);

    fn set_retransmission_time(&self, time: gst::ClockTime);

    fn set_reusable(&self, reusable: bool);

    fn set_shared(&self, shared: bool);

    //fn set_state(&self, state: gst::State, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 27 }) -> bool;

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool);

    fn set_suspend_mode(&self, mode: RTSPSuspendMode);

    fn set_transport_mode(&self, mode: RTSPTransportMode);

    //fn setup_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage, info: /*Ignored*/&mut SDPInfo) -> bool;

    fn suspend(&self) -> Result<(), glib::error::BoolError>;

    fn take_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: &P);

    fn unprepare(&self) -> Result<(), glib::error::BoolError>;

    fn unsuspend(&self) -> Result<(), glib::error::BoolError>;

    fn use_time_provider(&self, time_provider: bool);

    fn get_property_bind_mcast_address(&self) -> bool;

    fn set_property_bind_mcast_address(&self, bind_mcast_address: bool);

    fn get_property_eos_shutdown(&self) -> bool;

    fn get_property_max_mcast_ttl(&self) -> u32;

    fn set_property_max_mcast_ttl(&self, max_mcast_ttl: u32);

    fn get_property_reusable(&self) -> bool;

    fn get_property_shared(&self) -> bool;

    fn get_property_stop_on_disconnect(&self) -> bool;

    fn set_property_time_provider(&self, time_provider: bool);

    fn connect_new_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_new_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_prepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_removed_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_target_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unprepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bind_mcast_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_mcast_ttl_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reusable_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_time_provider_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPMedia>> RTSPMediaExt for O {
    fn collect_streams(&self) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_collect_streams(self.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn complete_pipeline(&self, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 10, id: 31 }) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_complete_pipeline() }
    //}

    fn create_stream<P: IsA<gst::Element>, Q: IsA<gst::Pad>>(&self, payloader: &P, pad: &Q) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_media_create_stream(self.as_ref().to_glib_none().0, payloader.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0))
        }
    }

    fn find_stream(&self, control: &str) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_media_find_stream(self.as_ref().to_glib_none().0, control.to_glib_none().0))
        }
    }

    fn get_address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_get_address_pool(self.as_ref().to_glib_none().0))
        }
    }

    fn get_base_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_base_time(self.as_ref().to_glib_none().0))
        }
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_get_buffer_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_clock(&self) -> Option<gst::Clock> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_get_clock(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_do_retransmission(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_do_retransmission(self.as_ref().to_glib_none().0))
        }
    }

    fn get_element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_get_element(self.as_ref().to_glib_none().0))
        }
    }

    fn get_latency(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_get_latency(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_max_mcast_ttl(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_get_max_mcast_ttl(self.as_ref().to_glib_none().0)
        }
    }

    fn get_multicast_iface(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_get_multicast_iface(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions> {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_get_permissions() }
    //}

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_profiles(self.as_ref().to_glib_none().0))
        }
    }

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_protocols(self.as_ref().to_glib_none().0))
        }
    }

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_publish_clock_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_range_string(&self, play: bool, unit: gst_rtsp::RTSPRangeUnit) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_get_range_string(self.as_ref().to_glib_none().0, play.to_glib(), unit.to_glib()))
        }
    }

    fn get_retransmission_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_retransmission_time(self.as_ref().to_glib_none().0))
        }
    }

    fn get_status(&self) -> RTSPMediaStatus {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_status(self.as_ref().to_glib_none().0))
        }
    }

    fn get_stream(&self, idx: u32) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_media_get_stream(self.as_ref().to_glib_none().0, idx))
        }
    }

    fn get_suspend_mode(&self) -> RTSPSuspendMode {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_suspend_mode(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_time_provider(&self, address: Option<&str>, port: u16) -> /*Ignored*/Option<gst_net::NetTimeProvider> {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_get_time_provider() }
    //}

    fn get_transport_mode(&self) -> RTSPTransportMode {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_get_transport_mode(self.as_ref().to_glib_none().0))
        }
    }

    //fn handle_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_handle_sdp() }
    //}

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn is_bind_mcast_address(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_bind_mcast_address(self.as_ref().to_glib_none().0))
        }
    }

    fn is_eos_shutdown(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_eos_shutdown(self.as_ref().to_glib_none().0))
        }
    }

    fn is_reusable(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_reusable(self.as_ref().to_glib_none().0))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_shared(self.as_ref().to_glib_none().0))
        }
    }

    fn is_stop_on_disconnect(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_stop_on_disconnect(self.as_ref().to_glib_none().0))
        }
    }

    fn is_time_provider(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_is_time_provider(self.as_ref().to_glib_none().0))
        }
    }

    fn n_streams(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_n_streams(self.as_ref().to_glib_none().0)
        }
    }

    //fn prepare(&self, thread: /*Ignored*/Option<&mut RTSPThread>) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_prepare() }
    //}

    //fn seek(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_seek() }
    //}

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn seek_full(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange, flags: /*Ignored*/gst::SeekFlags) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_seek_full() }
    //}

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn seekable(&self) -> /*Ignored*/gst::ClockTimeDiff {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_seekable() }
    //}

    fn set_address_pool<P: IsA<RTSPAddressPool>>(&self, pool: Option<&P>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_address_pool(self.as_ref().to_glib_none().0, pool.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_bind_mcast_address(self.as_ref().to_glib_none().0, bind_mcast_addr.to_glib());
        }
    }

    fn set_buffer_size(&self, size: u32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    fn set_clock<P: IsA<gst::Clock>>(&self, clock: Option<&P>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_clock(self.as_ref().to_glib_none().0, clock.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_do_retransmission(&self, do_retransmission: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_do_retransmission(self.as_ref().to_glib_none().0, do_retransmission.to_glib());
        }
    }

    fn set_eos_shutdown(&self, eos_shutdown: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_eos_shutdown(self.as_ref().to_glib_none().0, eos_shutdown.to_glib());
        }
    }

    fn set_latency(&self, latency: u32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_latency(self.as_ref().to_glib_none().0, latency);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_set_max_mcast_ttl(self.as_ref().to_glib_none().0, ttl))
        }
    }

    fn set_multicast_iface(&self, multicast_iface: Option<&str>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_multicast_iface(self.as_ref().to_glib_none().0, multicast_iface.to_glib_none().0);
        }
    }

    //fn set_permissions(&self, permissions: /*Ignored*/Option<&mut RTSPPermissions>) {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_set_permissions() }
    //}

    fn set_pipeline_state(&self, state: gst::State) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_pipeline_state(self.as_ref().to_glib_none().0, state.to_glib());
        }
    }

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_profiles(self.as_ref().to_glib_none().0, profiles.to_glib());
        }
    }

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_protocols(self.as_ref().to_glib_none().0, protocols.to_glib());
        }
    }

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_publish_clock_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_retransmission_time(&self, time: gst::ClockTime) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_retransmission_time(self.as_ref().to_glib_none().0, time.to_glib());
        }
    }

    fn set_reusable(&self, reusable: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_reusable(self.as_ref().to_glib_none().0, reusable.to_glib());
        }
    }

    fn set_shared(&self, shared: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_shared(self.as_ref().to_glib_none().0, shared.to_glib());
        }
    }

    //fn set_state(&self, state: gst::State, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 27 }) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_set_state() }
    //}

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_stop_on_disconnect(self.as_ref().to_glib_none().0, stop_on_disconnect.to_glib());
        }
    }

    fn set_suspend_mode(&self, mode: RTSPSuspendMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_suspend_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_transport_mode(&self, mode: RTSPTransportMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_set_transport_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    //fn setup_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage, info: /*Ignored*/&mut SDPInfo) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_setup_sdp() }
    //}

    fn suspend(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(gst_rtsp_server_sys::gst_rtsp_media_suspend(self.as_ref().to_glib_none().0), "Failed to suspend media")
        }
    }

    fn take_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: &P) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_take_pipeline(self.as_ref().to_glib_none().0, pipeline.as_ref().to_glib_full());
        }
    }

    fn unprepare(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(gst_rtsp_server_sys::gst_rtsp_media_unprepare(self.as_ref().to_glib_none().0), "Failed to unprepare media")
        }
    }

    fn unsuspend(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(gst_rtsp_server_sys::gst_rtsp_media_unsuspend(self.as_ref().to_glib_none().0), "Failed to unsuspend media")
        }
    }

    fn use_time_provider(&self, time_provider: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_use_time_provider(self.as_ref().to_glib_none().0, time_provider.to_glib());
        }
    }

    fn get_property_bind_mcast_address(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"bind-mcast-address\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_bind_mcast_address(&self, bind_mcast_address: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"bind-mcast-address\0".as_ptr() as *const _, Value::from(&bind_mcast_address).to_glib_none().0);
        }
    }

    fn get_property_eos_shutdown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"eos-shutdown\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_max_mcast_ttl(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"max-mcast-ttl\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_max_mcast_ttl(&self, max_mcast_ttl: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"max-mcast-ttl\0".as_ptr() as *const _, Value::from(&max_mcast_ttl).to_glib_none().0);
        }
    }

    fn get_property_reusable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"reusable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_shared(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"shared\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_stop_on_disconnect(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"stop-on-disconnect\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_time_provider(&self, time_provider: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"time-provider\0".as_ptr() as *const _, Value::from(&time_provider).to_glib_none().0);
        }
    }

    fn connect_new_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn new_state_trampoline<P, F: Fn(&P, i32) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, object: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"new-state\0".as_ptr() as *const _,
                Some(transmute(new_state_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_new_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn new_stream_trampoline<P, F: Fn(&P, &RTSPStream) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, object: *mut gst_rtsp_server_sys::GstRTSPStream, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"new-stream\0".as_ptr() as *const _,
                Some(transmute(new_stream_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_prepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prepared_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"prepared\0".as_ptr() as *const _,
                Some(transmute(prepared_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_removed_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_stream_trampoline<P, F: Fn(&P, &RTSPStream) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, object: *mut gst_rtsp_server_sys::GstRTSPStream, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"removed-stream\0".as_ptr() as *const _,
                Some(transmute(removed_stream_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_target_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn target_state_trampoline<P, F: Fn(&P, i32) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, object: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"target-state\0".as_ptr() as *const _,
                Some(transmute(target_state_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_unprepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unprepared_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"unprepared\0".as_ptr() as *const _,
                Some(transmute(unprepared_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_bind_mcast_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bind_mcast_address_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bind-mcast-address\0".as_ptr() as *const _,
                Some(transmute(notify_bind_mcast_address_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute(notify_buffer_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::clock\0".as_ptr() as *const _,
                Some(transmute(notify_clock_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_eos_shutdown_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::eos-shutdown\0".as_ptr() as *const _,
                Some(transmute(notify_eos_shutdown_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::latency\0".as_ptr() as *const _,
                Some(transmute(notify_latency_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_max_mcast_ttl_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_mcast_ttl_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-mcast-ttl\0".as_ptr() as *const _,
                Some(transmute(notify_max_mcast_ttl_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_profiles_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::profiles\0".as_ptr() as *const _,
                Some(transmute(notify_profiles_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocols_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::protocols\0".as_ptr() as *const _,
                Some(transmute(notify_protocols_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reusable_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reusable_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reusable\0".as_ptr() as *const _,
                Some(transmute(notify_reusable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shared_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shared\0".as_ptr() as *const _,
                Some(transmute(notify_shared_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stop_on_disconnect_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stop-on-disconnect\0".as_ptr() as *const _,
                Some(transmute(notify_stop_on_disconnect_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_suspend_mode_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::suspend-mode\0".as_ptr() as *const _,
                Some(transmute(notify_suspend_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_time_provider_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_time_provider_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::time-provider\0".as_ptr() as *const _,
                Some(transmute(notify_time_provider_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transport_mode_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_rtsp_server_sys::GstRTSPMedia, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<RTSPMedia>
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMedia::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transport-mode\0".as_ptr() as *const _,
                Some(transmute(notify_transport_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}
