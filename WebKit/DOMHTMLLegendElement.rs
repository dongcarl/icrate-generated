//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLLegendElement")]
    #[deprecated]
    pub struct DOMHTMLLegendElement;

    #[cfg(feature = "WebKit_DOMHTMLLegendElement")]
    unsafe impl ClassType for DOMHTMLLegendElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLLegendElement")]
    unsafe impl DOMHTMLLegendElement {
        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);
    }
);