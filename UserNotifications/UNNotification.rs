//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotification")]
    pub struct UNNotification;

    #[cfg(feature = "UserNotifications_UNNotification")]
    unsafe impl ClassType for UNNotification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNNotification")]
unsafe impl NSCoding for UNNotification {}

#[cfg(feature = "UserNotifications_UNNotification")]
unsafe impl NSCopying for UNNotification {}

#[cfg(feature = "UserNotifications_UNNotification")]
unsafe impl NSObjectProtocol for UNNotification {}

#[cfg(feature = "UserNotifications_UNNotification")]
unsafe impl NSSecureCoding for UNNotification {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotification")]
    unsafe impl UNNotification {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[cfg(feature = "UserNotifications_UNNotificationRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<UNNotificationRequest>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNNotification")]
    unsafe impl UNNotification {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
