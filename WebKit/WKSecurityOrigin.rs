//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKSecurityOrigin")]
    pub struct WKSecurityOrigin;

    #[cfg(feature = "WebKit_WKSecurityOrigin")]
    unsafe impl ClassType for WKSecurityOrigin {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKSecurityOrigin")]
unsafe impl NSObjectProtocol for WKSecurityOrigin {}

extern_methods!(
    #[cfg(feature = "WebKit_WKSecurityOrigin")]
    unsafe impl WKSecurityOrigin {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKSecurityOrigin")]
    unsafe impl WKSecurityOrigin {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
