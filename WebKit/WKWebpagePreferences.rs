//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKContentMode {
        WKContentModeRecommended = 0,
        WKContentModeMobile = 1,
        WKContentModeDesktop = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKWebpagePreferences")]
    pub struct WKWebpagePreferences;

    #[cfg(feature = "WebKit_WKWebpagePreferences")]
    unsafe impl ClassType for WKWebpagePreferences {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WKWebpagePreferences")]
    unsafe impl WKWebpagePreferences {
        #[method(preferredContentMode)]
        pub unsafe fn preferredContentMode(&self) -> WKContentMode;

        #[method(setPreferredContentMode:)]
        pub unsafe fn setPreferredContentMode(&self, preferred_content_mode: WKContentMode);

        #[method(allowsContentJavaScript)]
        pub unsafe fn allowsContentJavaScript(&self) -> bool;

        #[method(setAllowsContentJavaScript:)]
        pub unsafe fn setAllowsContentJavaScript(&self, allows_content_java_script: bool);

        #[method(isLockdownModeEnabled)]
        pub unsafe fn isLockdownModeEnabled(&self) -> bool;

        #[method(setLockdownModeEnabled:)]
        pub unsafe fn setLockdownModeEnabled(&self, lockdown_mode_enabled: bool);
    }
);