//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKContentRuleList")]
    pub struct WKContentRuleList;

    #[cfg(feature = "WebKit_WKContentRuleList")]
    unsafe impl ClassType for WKContentRuleList {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKContentRuleList")]
unsafe impl NSObjectProtocol for WKContentRuleList {}

extern_methods!(
    #[cfg(feature = "WebKit_WKContentRuleList")]
    unsafe impl WKContentRuleList {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKContentRuleList")]
    unsafe impl WKContentRuleList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
