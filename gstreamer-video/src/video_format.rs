// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_video_sys;

use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum VideoEndianness {
    Unknown,
    LittleEndian = 1234,
    BigEndian = 4321,
}

impl FromGlib<i32> for VideoEndianness {
    fn from_glib(value: i32) -> Self {
        skip_assert_initialized!();

        match value {
            1234 => VideoEndianness::LittleEndian,
            4321 => VideoEndianness::BigEndian,
            _ => VideoEndianness::Unknown,
        }
    }
}

impl ToGlib for VideoEndianness {
    type GlibType = i32;

    fn to_glib(&self) -> i32 {
        match *self {
            VideoEndianness::LittleEndian => 1234,
            VideoEndianness::BigEndian => 4321,
            _ => 0,
        }
    }
}

impl ::VideoFormat {
    pub fn from_fourcc(fourcc: u32) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe { from_glib(gst_video_sys::gst_video_format_from_fourcc(fourcc)) }
    }

    pub fn from_masks(
        depth: u32,
        bpp: u32,
        endianness: ::VideoEndianness,
        red_mask: u32,
        blue_mask: u32,
        green_mask: u32,
        alpha_mask: u32,
    ) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(gst_video_sys::gst_video_format_from_masks(
                depth as i32,
                bpp as i32,
                endianness.to_glib(),
                red_mask,
                blue_mask,
                green_mask,
                alpha_mask,
            ))
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        if self == ::VideoFormat::Unknown {
            return "UNKNOWN";
        }

        unsafe {
            CStr::from_ptr(gst_video_sys::gst_video_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }
}

impl str::FromStr for ::VideoFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let fmt = ::VideoFormat::from_glib(gst_video_sys::gst_video_format_from_string(
                s.to_glib_none().0,
            ));

            if fmt == ::VideoFormat::Unknown {
                Err(glib_bool_error!("Failed to parse video format from string"))
            } else {
                Ok(fmt)
            }
        }
    }
}

impl fmt::Display for ::VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}

#[cfg(test)]
mod tests {
    use gst;

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", ::VideoFormat::Nv16);
    }
}
