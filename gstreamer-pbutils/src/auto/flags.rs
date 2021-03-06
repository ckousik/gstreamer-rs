// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use gst_pbutils_sys;

bitflags! {
    pub struct DiscovererSerializeFlags: u32 {
        const BASIC = 0;
        const CAPS = 1;
        const TAGS = 2;
        const MISC = 4;
        const ALL = 7;
    }
}

#[doc(hidden)]
impl ToGlib for DiscovererSerializeFlags {
    type GlibType = gst_pbutils_sys::GstDiscovererSerializeFlags;

    fn to_glib(&self) -> gst_pbutils_sys::GstDiscovererSerializeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_pbutils_sys::GstDiscovererSerializeFlags> for DiscovererSerializeFlags {
    fn from_glib(value: gst_pbutils_sys::GstDiscovererSerializeFlags) -> DiscovererSerializeFlags {
        skip_assert_initialized!();
        DiscovererSerializeFlags::from_bits_truncate(value)
    }
}

impl StaticType for DiscovererSerializeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_pbutils_sys::gst_discoverer_serialize_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DiscovererSerializeFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DiscovererSerializeFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DiscovererSerializeFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
