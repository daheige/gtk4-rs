// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use gdk_sys;
use gio;
use glib;
use glib::GString;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ContentSerializer(Object<gdk_sys::GdkContentSerializer, ContentSerializerClass>);

    match fn {
        get_type => || gdk_sys::gdk_content_serializer_get_type(),
    }
}

impl ContentSerializer {
    pub fn get_cancellable(&self) -> Option<gio::Cancellable> {
        unsafe {
            from_glib_none(gdk_sys::gdk_content_serializer_get_cancellable(self.to_glib_none().0))
        }
    }

    pub fn get_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(gdk_sys::gdk_content_serializer_get_gtype(self.to_glib_none().0))
        }
    }

    pub fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_sys::gdk_content_serializer_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_output_stream(&self) -> Option<gio::OutputStream> {
        unsafe {
            from_glib_none(gdk_sys::gdk_content_serializer_get_output_stream(self.to_glib_none().0))
        }
    }

    pub fn get_priority(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_content_serializer_get_priority(self.to_glib_none().0)
        }
    }

    //pub fn get_task_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gdk_sys:gdk_content_serializer_get_task_data() }
    //}

    //pub fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gdk_sys:gdk_content_serializer_get_user_data() }
    //}

    pub fn get_value(&self) -> Option<glib::Value> {
        unsafe {
            from_glib_none(gdk_sys::gdk_content_serializer_get_value(self.to_glib_none().0))
        }
    }

    pub fn return_error(&self, error: &mut Error) {
        unsafe {
            gdk_sys::gdk_content_serializer_return_error(self.to_glib_none().0, error.to_glib_none_mut().0);
        }
    }

    pub fn return_success(&self) {
        unsafe {
            gdk_sys::gdk_content_serializer_return_success(self.to_glib_none().0);
        }
    }

    //pub fn set_task_data(&self, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gdk_sys:gdk_content_serializer_set_task_data() }
    //}
}

impl fmt::Display for ContentSerializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContentSerializer")
    }
}
