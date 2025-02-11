//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutRoute")]
    pub struct HKWorkoutRoute;

    #[cfg(feature = "HealthKit_HKWorkoutRoute")]
    unsafe impl ClassType for HKWorkoutRoute {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKSeriesSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutRoute")]
unsafe impl NSCoding for HKWorkoutRoute {}

#[cfg(feature = "HealthKit_HKWorkoutRoute")]
unsafe impl NSObjectProtocol for HKWorkoutRoute {}

#[cfg(feature = "HealthKit_HKWorkoutRoute")]
unsafe impl NSSecureCoding for HKWorkoutRoute {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutRoute")]
    unsafe impl HKWorkoutRoute {}
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKWorkoutRoute")]
    unsafe impl HKWorkoutRoute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKWorkoutRoute")]
    unsafe impl HKWorkoutRoute {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
