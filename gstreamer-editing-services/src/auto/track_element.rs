// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Edge;
use EditMode;
use Extractable;
use Layer;
use TimelineElement;
use Track;
use TrackType;
use ges_sys;
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
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TrackElement(Object<ges_sys::GESTrackElement, ges_sys::GESTrackElementClass, TrackElementClass>) @extends TimelineElement, @implements Extractable;

    match fn {
        get_type => || ges_sys::ges_track_element_get_type(),
    }
}

pub const NONE_TRACK_ELEMENT: Option<&TrackElement> = None;

pub trait TrackElementExt: 'static {
    fn add_children_props<P: IsA<gst::Element>>(&self, element: &P, wanted_categories: &[&str], blacklist: &[&str], whitelist: &[&str]);

    fn edit(&self, layers: &[Layer], mode: EditMode, edge: Edge, position: u64) -> Result<(), glib::error::BoolError>;

    //fn get_all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 83 };

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<gst::ControlBinding>;

    fn get_element(&self) -> Option<gst::Element>;

    fn get_gnlobject(&self) -> Option<gst::Element>;

    fn get_nleobject(&self) -> Option<gst::Element>;

    fn get_track(&self) -> Option<Track>;

    fn get_track_type(&self) -> TrackType;

    fn is_active(&self) -> bool;

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element>;

    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError>;

    fn set_active(&self, active: bool) -> bool;

    //fn set_control_source(&self, source: /*Ignored*/&gst::ControlSource, property_name: &str, binding_type: &str) -> bool;

    fn set_track_type(&self, type_: TrackType);

    fn get_property_active(&self) -> bool;

    //fn connect_control_binding_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_control_binding_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TrackElement>> TrackElementExt for O {
    fn add_children_props<P: IsA<gst::Element>>(&self, element: &P, wanted_categories: &[&str], blacklist: &[&str], whitelist: &[&str]) {
        unsafe {
            ges_sys::ges_track_element_add_children_props(self.as_ref().to_glib_none().0, element.as_ref().to_glib_none().0, wanted_categories.to_glib_none().0, blacklist.to_glib_none().0, whitelist.to_glib_none().0);
        }
    }

    fn edit(&self, layers: &[Layer], mode: EditMode, edge: Edge, position: u64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_track_element_edit(self.as_ref().to_glib_none().0, layers.to_glib_none().0, mode.to_glib(), edge.to_glib(), position), "Failed to edit")
        }
    }

    //fn get_all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 83 } {
    //    unsafe { TODO: call ges_sys:ges_track_element_get_all_control_bindings() }
    //}

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<gst::ControlBinding> {
    //    unsafe { TODO: call ges_sys:ges_track_element_get_control_binding() }
    //}

    fn get_element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ges_sys::ges_track_element_get_element(self.as_ref().to_glib_none().0))
        }
    }

    fn get_gnlobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ges_sys::ges_track_element_get_gnlobject(self.as_ref().to_glib_none().0))
        }
    }

    fn get_nleobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ges_sys::ges_track_element_get_nleobject(self.as_ref().to_glib_none().0))
        }
    }

    fn get_track(&self) -> Option<Track> {
        unsafe {
            from_glib_none(ges_sys::ges_track_element_get_track(self.as_ref().to_glib_none().0))
        }
    }

    fn get_track_type(&self) -> TrackType {
        unsafe {
            from_glib(ges_sys::ges_track_element_get_track_type(self.as_ref().to_glib_none().0))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_track_element_is_active(self.as_ref().to_glib_none().0))
        }
    }

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element> {
    //    unsafe { TODO: call ges_sys:ges_track_element_lookup_child() }
    //}

    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ges_sys::ges_track_element_remove_control_binding(self.as_ref().to_glib_none().0, property_name.to_glib_none().0), "Failed to remove control binding")
        }
    }

    fn set_active(&self, active: bool) -> bool {
        unsafe {
            from_glib(ges_sys::ges_track_element_set_active(self.as_ref().to_glib_none().0, active.to_glib()))
        }
    }

    //fn set_control_source(&self, source: /*Ignored*/&gst::ControlSource, property_name: &str, binding_type: &str) -> bool {
    //    unsafe { TODO: call ges_sys:ges_track_element_set_control_source() }
    //}

    fn set_track_type(&self, type_: TrackType) {
        unsafe {
            ges_sys::ges_track_element_set_track_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"active\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    //fn connect_control_binding_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored control_binding: Gst.ControlBinding
    //}

    //fn connect_control_binding_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored control_binding: Gst.ControlBinding
    //}

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrackElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<TrackElement>
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrackElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<TrackElement>
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::track\0".as_ptr() as *const _,
                Some(transmute(notify_track_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ges_sys::GESTrackElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<TrackElement>
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::track-type\0".as_ptr() as *const _,
                Some(transmute(notify_track_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}
