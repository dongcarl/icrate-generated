//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLookAroundScene")]
    pub struct MKLookAroundScene;

    #[cfg(feature = "MapKit_MKLookAroundScene")]
    unsafe impl ClassType for MKLookAroundScene {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLookAroundScene")]
unsafe impl NSCopying for MKLookAroundScene {}

#[cfg(feature = "MapKit_MKLookAroundScene")]
unsafe impl NSObjectProtocol for MKLookAroundScene {}

extern_methods!(
    #[cfg(feature = "MapKit_MKLookAroundScene")]
    unsafe impl MKLookAroundScene {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
