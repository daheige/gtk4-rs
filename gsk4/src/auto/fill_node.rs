// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FillRule, Path, RenderNode};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GskFillNode")]
    pub struct FillNode(Shared<ffi::GskFillNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for FillNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_fill_node_get_type()) }
    }
}

impl FillNode {
    #[doc(alias = "gsk_fill_node_new")]
    pub fn new(child: impl AsRef<RenderNode>, path: &Path, fill_rule: FillRule) -> FillNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gsk_fill_node_new(
                child.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                fill_rule.into_glib(),
            ))
        }
    }

    #[doc(alias = "gsk_fill_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_fill_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_fill_node_get_fill_rule")]
    #[doc(alias = "get_fill_rule")]
    pub fn fill_rule(&self) -> FillRule {
        unsafe { from_glib(ffi::gsk_fill_node_get_fill_rule(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_fill_node_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Path {
        unsafe { from_glib_none(ffi::gsk_fill_node_get_path(self.to_glib_none().0)) }
    }
}
