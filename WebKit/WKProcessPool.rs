//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKProcessPool")]
    pub struct WKProcessPool;

    #[cfg(feature = "WebKit_WKProcessPool")]
    unsafe impl ClassType for WKProcessPool {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKProcessPool")]
unsafe impl NSCoding for WKProcessPool {}

#[cfg(feature = "WebKit_WKProcessPool")]
unsafe impl NSObjectProtocol for WKProcessPool {}

#[cfg(feature = "WebKit_WKProcessPool")]
unsafe impl NSSecureCoding for WKProcessPool {}

extern_methods!(
    #[cfg(feature = "WebKit_WKProcessPool")]
    unsafe impl WKProcessPool {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKProcessPool")]
    unsafe impl WKProcessPool {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
