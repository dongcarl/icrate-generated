//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    pub struct ILCallClassificationRequest;

    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    unsafe impl ClassType for ILCallClassificationRequest {
        #[inherits(NSObject)]
        type Super = ILClassificationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSCoding for ILCallClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSObjectProtocol for ILCallClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSSecureCoding for ILCallClassificationRequest {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    unsafe impl ILCallClassificationRequest {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "IdentityLookup_ILCallCommunication"
        ))]
        #[method_id(@__retain_semantics Other callCommunications)]
        pub unsafe fn callCommunications(&self) -> Id<NSArray<ILCallCommunication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    unsafe impl ILCallClassificationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
