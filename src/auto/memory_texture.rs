// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MemoryFormat;
use Paintable;
use Texture;
use gdk_sys;
use glib;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct MemoryTexture(Object<gdk_sys::GdkMemoryTexture, gdk_sys::GdkMemoryTextureClass, MemoryTextureClass>) @extends Texture, @implements Paintable;

    match fn {
        get_type => || gdk_sys::gdk_memory_texture_get_type(),
    }
}

impl MemoryTexture {
    pub fn new(width: i32, height: i32, format: MemoryFormat, bytes: &glib::Bytes, stride: usize) -> MemoryTexture {
        assert_initialized_main_thread!();
        unsafe {
            Texture::from_glib_full(gdk_sys::gdk_memory_texture_new(width, height, format.to_glib(), bytes.to_glib_none().0, stride)).unsafe_cast()
        }
    }
}

impl fmt::Display for MemoryTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemoryTexture")
    }
}
