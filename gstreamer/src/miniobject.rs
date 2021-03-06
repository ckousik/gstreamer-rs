// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::{borrow, fmt, ops};

use glib;
use glib::translate::*;
use glib_sys;
use glib_sys::gpointer;
use gobject_sys;
use gst_sys;

pub struct GstRc<T: MiniObject> {
    obj: ptr::NonNull<T>,
    phantom: PhantomData<T>,
}

impl<T: MiniObject> GstRc<T> {
    pub unsafe fn from_glib_none(ptr: *const T::GstType) -> Self {
        skip_assert_initialized!();
        assert!(!ptr.is_null());

        gst_sys::gst_mini_object_ref(ptr as *mut gst_sys::GstMiniObject);

        GstRc {
            obj: ptr::NonNull::new_unchecked(ptr as *mut T::GstType as *mut T),
            phantom: PhantomData,
        }
    }

    pub unsafe fn from_glib_full(ptr: *const T::GstType) -> Self {
        assert!(!ptr.is_null());

        GstRc {
            obj: ptr::NonNull::new_unchecked(ptr as *mut T::GstType as *mut T),
            phantom: PhantomData,
        }
    }

    pub unsafe fn from_glib_borrow(ptr: *const T::GstType) -> Borrowed<Self> {
        assert!(!ptr.is_null());

        Borrowed::new(GstRc {
            obj: ptr::NonNull::new_unchecked(ptr as *mut T::GstType as *mut T),
            phantom: PhantomData,
        })
    }

    pub unsafe fn replace_ptr(&mut self, ptr: *mut T::GstType) {
        assert!(!ptr.is_null());
        self.obj = ptr::NonNull::new_unchecked(ptr as *mut T);
    }

    pub fn make_mut(&mut self) -> &mut T {
        unsafe {
            if self.is_writable() {
                return self.obj.as_mut();
            }

            let ptr = gst_sys::gst_mini_object_make_writable(
                self.as_mut_ptr() as *mut gst_sys::GstMiniObject
            );
            self.replace_ptr(ptr as *mut T::GstType);
            assert!(self.is_writable());

            self.obj.as_mut()
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_writable() {
            Some(unsafe { self.obj.as_mut() })
        } else {
            None
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_mini_object_is_writable(
                self.as_ptr() as *const gst_sys::GstMiniObject
            ))
        }
    }

    pub unsafe fn into_ptr(self) -> *mut T::GstType {
        let s = mem::ManuallyDrop::new(self);
        s.as_mut_ptr()
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn add_parent<U: MiniObject>(&self, parent: &U) {
        unsafe {
            gst_sys::gst_mini_object_add_parent(
                self.as_ptr() as *mut gst_sys::GstMiniObject,
                parent.as_ptr() as *mut gst_sys::GstMiniObject,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn remove_parent<U: MiniObject>(&self, parent: &U) {
        unsafe {
            gst_sys::gst_mini_object_remove_parent(
                self.as_ptr() as *mut gst_sys::GstMiniObject,
                parent.as_ptr() as *mut gst_sys::GstMiniObject,
            );
        }
    }
}

impl<T: MiniObject> ops::Deref for GstRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T: MiniObject> AsRef<T> for GstRc<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.obj.as_ref() }
    }
}

impl<T: MiniObject> borrow::Borrow<T> for GstRc<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: MiniObject> Clone for GstRc<T> {
    fn clone(&self) -> GstRc<T> {
        unsafe { GstRc::from_glib_none(self.as_ptr()) }
    }
}

impl<T: MiniObject> Drop for GstRc<T> {
    fn drop(&mut self) {
        unsafe {
            gst_sys::gst_mini_object_unref(self.as_mut_ptr() as *mut gst_sys::GstMiniObject);
        }
    }
}

unsafe impl<T: MiniObject + Sync + Send> Sync for GstRc<T> {}
unsafe impl<T: MiniObject + Sync + Send> Send for GstRc<T> {}

impl<T: MiniObject + PartialEq> PartialEq for GstRc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T: MiniObject + Eq> Eq for GstRc<T> {}

impl<T: MiniObject + fmt::Debug> fmt::Debug for GstRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: MiniObject + fmt::Display> fmt::Display for GstRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

pub unsafe trait MiniObject
where
    Self: Sized,
{
    type GstType;

    unsafe fn as_ptr(&self) -> *const Self::GstType {
        self as *const Self as *const Self::GstType
    }

    unsafe fn as_mut_ptr(&self) -> *mut Self::GstType {
        self as *const Self as *mut Self::GstType
    }

    unsafe fn from_ptr<'a>(ptr: *const Self::GstType) -> &'a Self {
        assert!(!ptr.is_null());
        &*(ptr as *const Self)
    }

    unsafe fn from_mut_ptr<'a>(ptr: *mut Self::GstType) -> &'a mut Self {
        assert!(!ptr.is_null());
        assert_ne!(
            gst_sys::gst_mini_object_is_writable(ptr as *mut gst_sys::GstMiniObject),
            glib_sys::GFALSE
        );
        &mut *(ptr as *mut Self)
    }

    fn copy(&self) -> GstRc<Self> {
        unsafe {
            GstRc::from_glib_full(gst_sys::gst_mini_object_copy(
                self.as_ptr() as *const gst_sys::GstMiniObject
            ) as *const Self::GstType)
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtr<'a, *const T::GstType> for GstRc<T> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const T::GstType, Self> {
        Stash(unsafe { self.as_ptr() }, self)
    }

    fn to_glib_full(&self) -> *const T::GstType {
        unsafe {
            gst_sys::gst_mini_object_ref(self.as_mut_ptr() as *mut gst_sys::GstMiniObject);
            self.as_ptr()
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtr<'a, *mut T::GstType> for GstRc<T> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut T::GstType, Self> {
        Stash(unsafe { self.as_mut_ptr() }, self)
    }

    fn to_glib_full(&self) -> *mut T::GstType {
        unsafe {
            gst_sys::gst_mini_object_ref(self.as_mut_ptr() as *mut gst_sys::GstMiniObject);
            self.as_mut_ptr()
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtrMut<'a, *mut T::GstType> for GstRc<T> {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut T::GstType, Self> {
        self.make_mut();
        StashMut(unsafe { self.as_mut_ptr() }, self)
    }
}

impl<'a, T: MiniObject + 'static> ToGlibContainerFromSlice<'a, *mut *mut T::GstType> for GstRc<T> {
    #[allow(clippy::type_complexity)]
    type Storage = (
        Vec<Stash<'a, *mut T::GstType, GstRc<T>>>,
        Option<Vec<*mut T::GstType>>,
    );

    fn to_glib_none_from_slice(t: &'a [GstRc<T>]) -> (*mut *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut() as *mut T::GstType);

        (v_ptr.as_ptr() as *mut *mut T::GstType, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [GstRc<T>]) -> (*mut *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<*mut T::GstType>() * t.len() + 1)
                as *mut *mut T::GstType;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(t: &[GstRc<T>]) -> *mut *mut T::GstType {
        skip_assert_initialized!();
        unsafe {
            let v_ptr = glib_sys::g_malloc0(mem::size_of::<*mut T::GstType>() * t.len() + 1)
                as *mut *mut T::GstType;

            for (i, s) in t.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.to_glib_full());
            }

            v_ptr
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibContainerFromSlice<'a, *const *mut T::GstType>
    for GstRc<T>
{
    #[allow(clippy::type_complexity)]
    type Storage = (
        Vec<Stash<'a, *mut T::GstType, GstRc<T>>>,
        Option<Vec<*mut T::GstType>>,
    );

    fn to_glib_none_from_slice(t: &'a [GstRc<T>]) -> (*const *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let (ptr, stash) =
            ToGlibContainerFromSlice::<'a, *mut *mut T::GstType>::to_glib_none_from_slice(t);
        (ptr as *const *mut T::GstType, stash)
    }

    fn to_glib_container_from_slice(_: &'a [GstRc<T>]) -> (*const *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        // Can't have consumer free a *const pointer
        unimplemented!()
    }

    fn to_glib_full_from_slice(_: &[GstRc<T>]) -> *const *mut T::GstType {
        skip_assert_initialized!();
        // Can't have consumer free a *const pointer
        unimplemented!()
    }
}

impl<T: MiniObject + 'static> FromGlibPtrNone<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_none(ptr: *const T::GstType) -> Self {
        Self::from_glib_none(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrNone<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_none(ptr: *mut T::GstType) -> Self {
        Self::from_glib_none(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrFull<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_full(ptr: *const T::GstType) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrFull<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_full(ptr: *mut T::GstType) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrBorrow<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_borrow(ptr: *const T::GstType) -> Borrowed<Self> {
        Self::from_glib_borrow(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrBorrow<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_borrow(ptr: *mut T::GstType) -> Borrowed<Self> {
        Self::from_glib_borrow(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibContainerAsVec<*mut T::GstType, *mut *mut T::GstType>
    for GstRc<T>
{
    unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_full(ptr::read(ptr.add(i))));
        }
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

impl<T: MiniObject + 'static> FromGlibPtrArrayContainerAsVec<*mut T::GstType, *mut *mut T::GstType>
    for GstRc<T>
{
    unsafe fn from_glib_none_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(
            ptr,
            glib::translate::c_ptr_array_len(ptr),
        )
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(
            ptr,
            glib::translate::c_ptr_array_len(ptr),
        )
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(
            ptr,
            glib::translate::c_ptr_array_len(ptr),
        )
    }
}

impl<T: MiniObject + 'static> FromGlibContainerAsVec<*mut T::GstType, *const *mut T::GstType>
    for GstRc<T>
{
    unsafe fn from_glib_none_num_as_vec(ptr: *const *mut T::GstType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const *mut T::GstType, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const *mut T::GstType, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

impl<T: MiniObject + 'static>
    FromGlibPtrArrayContainerAsVec<*mut T::GstType, *const *mut T::GstType> for GstRc<T>
{
    unsafe fn from_glib_none_as_vec(ptr: *const *mut T::GstType) -> Vec<Self> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }

    unsafe fn from_glib_container_as_vec(_: *const *mut T::GstType) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const *mut T::GstType) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

impl<T: MiniObject + glib::StaticType> glib::StaticType for GstRc<T> {
    fn static_type() -> glib::types::Type {
        T::static_type()
    }
}

impl<'a, T: MiniObject + glib::StaticType + 'static> glib::value::FromValueOptional<'a>
    for GstRc<T>
{
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_sys::g_value_get_boxed(v.to_glib_none().0);
        from_glib_none(ptr as *const T::GstType)
    }
}

impl<T: MiniObject + glib::StaticType> glib::value::SetValue for GstRc<T> {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
    }
}

impl<T: MiniObject + glib::StaticType> glib::value::SetValueOptional for GstRc<T> {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

impl<T: MiniObject + 'static> GlibPtrDefault for GstRc<T> {
    type GlibType = *mut T::GstType;
}

#[macro_export]
macro_rules! gst_define_mini_object_wrapper(
    ($name:ident, $ref_name:ident, $gst_sys_name:path, [$($derives:ident,)*], $get_type:expr) => {
        #[derive($($derives,)*)]
        #[derive(Clone)]
        pub struct $name($crate::GstRc<$ref_name>);

        #[repr(C)]
        pub struct $ref_name($gst_sys_name);

        impl $name {
            pub unsafe fn from_glib_none(ptr: *const $gst_sys_name) -> Self {
                $name($crate::glib::translate::from_glib_none(ptr))
            }

            pub unsafe fn from_glib_full(ptr: *const $gst_sys_name) -> Self {
                $name($crate::glib::translate::from_glib_full(ptr))
            }

            pub unsafe fn from_glib_borrow(ptr: *const $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                $crate::glib::translate::Borrowed::new(
                    $name(
                        $crate::glib::translate::from_glib_borrow::<_, $crate::GstRc::<$ref_name>>(ptr)
                        .into_inner()
                        )
                    )
            }

            pub unsafe fn into_ptr(self) -> *mut $gst_sys_name {
                self.0.into_ptr()
            }

            pub fn copy(&self) -> Self {
                self.0.copy()
            }
        }

        impl $ref_name {
            fn copy(&self) -> $name {
                $name(<$ref_name as $crate::MiniObject>::copy(self))
            }
        }

        impl From<$crate::GstRc<$ref_name>> for $name {
            fn from(rc: $crate::GstRc<$ref_name>) -> $name {
	        skip_assert_initialized!();
                $name(rc)
            }
        }

        impl Into<$crate::GstRc<$ref_name>> for $name {
            fn into(self) -> $crate::GstRc<$ref_name> {
                self.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $crate::GstRc<$ref_name>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl AsRef<$ref_name> for $name {
            fn as_ref(&self) -> &$ref_name {
                self.0.as_ref()
            }
        }

        impl ::std::borrow::Borrow<$ref_name> for $name {
            fn borrow(&self) -> &$ref_name {
                self.as_ref()
            }
        }

        impl $crate::glib::types::StaticType for $name {
            fn static_type() -> $crate::glib::types::Type {
                $ref_name::static_type()
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *const $gst_sys_name> for $name {
            type Storage = $crate::glib::translate::Stash<'a, *const $gst_sys_name, $crate::GstRc<$ref_name>>;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *const $gst_sys_name, Self> {
                let stash = $crate::glib::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::glib::translate::Stash(stash.0, stash)
            }

            fn to_glib_full(&self) -> *const $gst_sys_name {
                $crate::glib::translate::ToGlibPtr::to_glib_full(&self.0)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *mut $gst_sys_name> for $name {
            type Storage = $crate::glib::translate::Stash<'a, *mut $gst_sys_name, $crate::GstRc<$ref_name>>;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *mut $gst_sys_name, Self> {
                let stash = $crate::glib::translate::ToGlibPtr::to_glib_none(&self.0);
                $crate::glib::translate::Stash(stash.0, stash)
            }

            fn to_glib_full(&self) -> *mut $gst_sys_name {
                $crate::glib::translate::ToGlibPtr::to_glib_full(&self.0)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtrMut<'a, *mut $gst_sys_name> for $name {
            type Storage = $crate::glib::translate::StashMut<'a, *mut $gst_sys_name, $crate::GstRc<$ref_name>>;

            fn to_glib_none_mut(&'a mut self) -> $crate::glib::translate::StashMut<*mut $gst_sys_name, Self> {
                let stash = $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut self.0);
                $crate::glib::translate::StashMut(stash.0, stash)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *mut *mut $gst_sys_name> for $name {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $gst_sys_name, $name>>,
                Option<Vec<*mut $gst_sys_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(::std::ptr::null_mut() as *mut $gst_sys_name);

                (v_ptr.as_ptr() as *mut *mut $gst_sys_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $gst_sys_name>() * t.len() + 1)
                        as *mut *mut $gst_sys_name;

                    for (i, s) in v.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $gst_sys_name {
                skip_assert_initialized!();
                unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $gst_sys_name>() * t.len() + 1)
                        as *mut *mut $gst_sys_name;

                    for (i, s) in t.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), $crate::glib::translate::ToGlibPtr::to_glib_full(&s));
                    }

                    v_ptr
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *const *mut $gst_sys_name>
            for $name
        {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $gst_sys_name, $name>>,
                Option<Vec<*mut $gst_sys_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let (ptr, stash) =
                    $crate::glib::translate::ToGlibContainerFromSlice::<'a, *mut *mut $gst_sys_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $gst_sys_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *mut $gst_sys_name {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*const $gst_sys_name> for $name {
            unsafe fn from_glib_none(ptr: *const $gst_sys_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_none(ptr: *mut $gst_sys_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*const $gst_sys_name> for $name {
            unsafe fn from_glib_full(ptr: *const $gst_sys_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_full(ptr: *mut $gst_sys_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*const $gst_sys_name> for $name {
            unsafe fn from_glib_borrow(ptr: *const $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_borrow(ptr: *mut $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $gst_sys_name, *mut *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_none(::std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                let res = $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_full(::std::ptr::read(ptr.add(i))));
                }
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $gst_sys_name, *mut *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $gst_sys_name, *const *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $gst_sys_name, num: usize) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $gst_sys_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $gst_sys_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $gst_sys_name, *const *mut $gst_sys_name> for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $gst_sys_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $gst_sys_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl<'a> $crate::glib::value::FromValueOptional<'a>
            for $name
        {
            unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
                <$crate::GstRc<$ref_name> as $crate::glib::value::FromValueOptional>::from_value_optional(v).map($name)
            }
        }

        impl $crate::glib::value::SetValue for $name {
            unsafe fn set_value(v: &mut glib::Value, s: &Self) {
                <$crate::GstRc<$ref_name> as $crate::glib::value::SetValue>::set_value(v, &s.0)
            }
        }

        impl $crate::glib::value::SetValueOptional for $name {
            unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
                <$crate::GstRc<$ref_name> as $crate::glib::value::SetValueOptional>::set_value_optional(v, s.map(|s| &s.0))
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $name {
            type GlibType = *mut $gst_sys_name;
        }

        unsafe impl $crate::MiniObject for $ref_name {
            type GstType = $gst_sys_name;
        }

        impl $crate::glib::types::StaticType for $ref_name {
            fn static_type() -> $crate::glib::types::Type {
                unsafe { $crate::glib::translate::from_glib($get_type()) }
            }
        }

        impl ToOwned for $ref_name {
            type Owned = $name;

            fn to_owned(&self) -> $name {
                self.copy()
            }
        }

        unsafe impl Sync for $ref_name {}
        unsafe impl Send for $ref_name {}
    }
);
