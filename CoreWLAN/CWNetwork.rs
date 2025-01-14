//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreWLAN::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreWLAN_CWNetwork")]
    pub struct CWNetwork;

    #[cfg(feature = "CoreWLAN_CWNetwork")]
    unsafe impl ClassType for CWNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreWLAN_CWNetwork")]
unsafe impl NSCoding for CWNetwork {}

#[cfg(feature = "CoreWLAN_CWNetwork")]
unsafe impl NSCopying for CWNetwork {}

#[cfg(feature = "CoreWLAN_CWNetwork")]
unsafe impl NSObjectProtocol for CWNetwork {}

#[cfg(feature = "CoreWLAN_CWNetwork")]
unsafe impl NSSecureCoding for CWNetwork {}

extern_methods!(
    #[cfg(feature = "CoreWLAN_CWNetwork")]
    unsafe impl CWNetwork {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bssid)]
        pub unsafe fn bssid(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CoreWLAN_CWChannel")]
        #[method_id(@__retain_semantics Other wlanChannel)]
        pub unsafe fn wlanChannel(&self) -> Option<Id<CWChannel>>;

        #[method(rssiValue)]
        pub unsafe fn rssiValue(&self) -> NSInteger;

        #[method(noiseMeasurement)]
        pub unsafe fn noiseMeasurement(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other informationElementData)]
        pub unsafe fn informationElementData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;

        #[method(beaconInterval)]
        pub unsafe fn beaconInterval(&self) -> NSInteger;

        #[method(ibss)]
        pub unsafe fn ibss(&self) -> bool;

        #[method(isEqualToNetwork:)]
        pub unsafe fn isEqualToNetwork(&self, network: &CWNetwork) -> bool;

        #[method(supportsSecurity:)]
        pub unsafe fn supportsSecurity(&self, security: CWSecurity) -> bool;

        #[method(supportsPHYMode:)]
        pub unsafe fn supportsPHYMode(&self, phy_mode: CWPHYMode) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreWLAN_CWNetwork")]
    unsafe impl CWNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
