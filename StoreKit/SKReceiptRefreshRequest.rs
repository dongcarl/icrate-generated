//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKReceiptRefreshRequest")]
    pub struct SKReceiptRefreshRequest;

    #[cfg(feature = "StoreKit_SKReceiptRefreshRequest")]
    unsafe impl ClassType for SKReceiptRefreshRequest {
        #[inherits(NSObject)]
        type Super = SKRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKReceiptRefreshRequest")]
unsafe impl NSObjectProtocol for SKReceiptRefreshRequest {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKReceiptRefreshRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithReceiptProperties:)]
        pub unsafe fn initWithReceiptProperties(
            this: Allocated<Self>,
            properties: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other receiptProperties)]
        pub unsafe fn receiptProperties(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKReceiptRefreshRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_fn!(
    pub unsafe fn SKTerminateForInvalidReceipt();
);

extern_static!(SKReceiptPropertyIsExpired: &'static NSString);

extern_static!(SKReceiptPropertyIsRevoked: &'static NSString);

extern_static!(SKReceiptPropertyIsVolumePurchase: &'static NSString);
