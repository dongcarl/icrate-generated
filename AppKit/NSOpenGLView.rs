//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSOpenGLSurfaceResolution
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[deprecated = "Use NSOpenGLView instead."]
        #[method(wantsBestResolutionOpenGLSurface)]
        pub unsafe fn wantsBestResolutionOpenGLSurface(&self) -> bool;

        #[deprecated = "Use NSOpenGLView instead."]
        #[method(setWantsBestResolutionOpenGLSurface:)]
        pub fn setWantsBestResolutionOpenGLSurface(
            &self,
            wants_best_resolution_open_gl_surface: bool,
        );
    }
);

extern_methods!(
    /// NSExtendedDynamicRange
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[deprecated = "Use NSOpenGLView instead."]
        #[method(wantsExtendedDynamicRangeOpenGLSurface)]
        pub unsafe fn wantsExtendedDynamicRangeOpenGLSurface(&self) -> bool;

        #[deprecated = "Use NSOpenGLView instead."]
        #[method(setWantsExtendedDynamicRangeOpenGLSurface:)]
        pub unsafe fn setWantsExtendedDynamicRangeOpenGLSurface(
            &self,
            wants_extended_dynamic_range_open_gl_surface: bool,
        );
    }
);
