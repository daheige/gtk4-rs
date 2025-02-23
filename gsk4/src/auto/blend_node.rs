// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{BlendMode, RenderNode};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GskBlendNode")]
    pub struct BlendNode(Shared<ffi::GskBlendNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for BlendNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_blend_node_get_type()) }
    }
}

impl BlendNode {
    #[doc(alias = "gsk_blend_node_new")]
    pub fn new(
        bottom: impl AsRef<RenderNode>,
        top: impl AsRef<RenderNode>,
        blend_mode: BlendMode,
    ) -> BlendNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_blend_node_new(
                bottom.as_ref().to_glib_none().0,
                top.as_ref().to_glib_none().0,
                blend_mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "gsk_blend_node_get_blend_mode")]
    #[doc(alias = "get_blend_mode")]
    pub fn blend_mode(&self) -> BlendMode {
        unsafe { from_glib(ffi::gsk_blend_node_get_blend_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_blend_node_get_bottom_child")]
    #[doc(alias = "get_bottom_child")]
    pub fn bottom_child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_bottom_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_blend_node_get_top_child")]
    #[doc(alias = "get_top_child")]
    pub fn top_child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_top_child(self.to_glib_none().0)) }
    }
}
