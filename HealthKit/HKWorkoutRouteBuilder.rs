//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
    pub struct HKWorkoutRouteBuilder;

    #[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
    unsafe impl ClassType for HKWorkoutRouteBuilder {
        #[inherits(NSObject)]
        type Super = HKSeriesBuilder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
unsafe impl NSObjectProtocol for HKWorkoutRouteBuilder {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[cfg(all(feature = "HealthKit_HKDevice", feature = "HealthKit_HKHealthStore"))]
        #[method_id(@__retain_semantics Init initWithHealthStore:device:)]
        pub unsafe fn initWithHealthStore_device(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            device: Option<&HKDevice>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(insertRouteData:completion:)]
        pub unsafe fn insertRouteData_completion(
            &self,
            route_data: &NSArray<CLLocation>,
            completion: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(addMetadata:completion:)]
        pub unsafe fn addMetadata_completion(
            &self,
            metadata: &NSDictionary<NSString, AnyObject>,
            completion: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKWorkout",
            feature = "HealthKit_HKWorkoutRoute"
        ))]
        #[method(finishRouteWithWorkout:metadata:completion:)]
        pub unsafe fn finishRouteWithWorkout_metadata_completion(
            &self,
            workout: &HKWorkout,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            completion: &Block<(*mut HKWorkoutRoute, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `HKSeriesBuilder`
    #[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKWorkoutRouteBuilder")]
    unsafe impl HKWorkoutRouteBuilder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
