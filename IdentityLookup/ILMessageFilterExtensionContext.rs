//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    pub struct ILMessageFilterExtensionContext;

    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    unsafe impl ClassType for ILMessageFilterExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
unsafe impl NSObjectProtocol for ILMessageFilterExtensionContext {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    unsafe impl ILMessageFilterExtensionContext {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "IdentityLookup_ILNetworkResponse"
        ))]
        #[method(deferQueryRequestToNetworkWithCompletion:)]
        pub unsafe fn deferQueryRequestToNetworkWithCompletion(
            &self,
            completion: &Block<(*mut ILNetworkResponse, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    unsafe impl ILMessageFilterExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
