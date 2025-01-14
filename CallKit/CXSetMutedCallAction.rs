//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    pub struct CXSetMutedCallAction;

    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl ClassType for CXSetMutedCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSCoding for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSCopying for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSObjectProtocol for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
unsafe impl NSSecureCoding for CXSetMutedCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl CXSetMutedCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:muted:)]
        pub unsafe fn initWithCallUUID_muted(
            this: Allocated<Self>,
            call_uuid: &NSUUID,
            muted: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder)
            -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID) -> Id<Self>;

        #[method(isMuted)]
        pub unsafe fn isMuted(&self) -> bool;

        #[method(setMuted:)]
        pub unsafe fn setMuted(&self, muted: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl CXSetMutedCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl CXSetMutedCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
