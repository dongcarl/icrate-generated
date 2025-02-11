//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    pub struct LAPersistedRight;

    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl ClassType for LAPersistedRight {
        #[inherits(NSObject)]
        type Super = LARight;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "LocalAuthentication_LAPersistedRight")]
unsafe impl NSObjectProtocol for LAPersistedRight {}

extern_methods!(
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl LAPersistedRight {
        #[cfg(feature = "LocalAuthentication_LAPrivateKey")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<LAPrivateKey>;

        #[cfg(feature = "LocalAuthentication_LASecret")]
        #[method_id(@__retain_semantics Other secret)]
        pub unsafe fn secret(&self) -> Id<LASecret>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `LARight`
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl LAPersistedRight {
        #[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Id<Self>;
    }
);
