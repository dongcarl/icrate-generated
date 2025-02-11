//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AdSupport::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AdSupport_ASIdentifierManager")]
    pub struct ASIdentifierManager;

    #[cfg(feature = "AdSupport_ASIdentifierManager")]
    unsafe impl ClassType for ASIdentifierManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AdSupport_ASIdentifierManager")]
unsafe impl NSObjectProtocol for ASIdentifierManager {}

extern_methods!(
    #[cfg(feature = "AdSupport_ASIdentifierManager")]
    unsafe impl ASIdentifierManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Id<ASIdentifierManager>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other advertisingIdentifier)]
        pub unsafe fn advertisingIdentifier(&self) -> Id<NSUUID>;

        #[deprecated = "This has been replaced by functionality in AppTrackingTransparency's ATTrackingManager class."]
        #[method(isAdvertisingTrackingEnabled)]
        pub unsafe fn isAdvertisingTrackingEnabled(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AdSupport_ASIdentifierManager")]
    unsafe impl ASIdentifierManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
