//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreWLAN::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait CWEventDelegate {
        #[optional]
        #[method(clientConnectionInterrupted)]
        unsafe fn clientConnectionInterrupted(&self);

        #[optional]
        #[method(clientConnectionInvalidated)]
        unsafe fn clientConnectionInvalidated(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(powerStateDidChangeForWiFiInterfaceWithName:)]
        unsafe fn powerStateDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(ssidDidChangeForWiFiInterfaceWithName:)]
        unsafe fn ssidDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(bssidDidChangeForWiFiInterfaceWithName:)]
        unsafe fn bssidDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(countryCodeDidChangeForWiFiInterfaceWithName:)]
        unsafe fn countryCodeDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(linkDidChangeForWiFiInterfaceWithName:)]
        unsafe fn linkDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(linkQualityDidChangeForWiFiInterfaceWithName:rssi:transmitRate:)]
        unsafe fn linkQualityDidChangeForWiFiInterfaceWithName_rssi_transmitRate(
            &self,
            interface_name: &NSString,
            rssi: NSInteger,
            transmit_rate: c_double,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(modeDidChangeForWiFiInterfaceWithName:)]
        unsafe fn modeDidChangeForWiFiInterfaceWithName(&self, interface_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(scanCacheUpdatedForWiFiInterfaceWithName:)]
        unsafe fn scanCacheUpdatedForWiFiInterfaceWithName(&self, interface_name: &NSString);
    }

    unsafe impl ProtocolType for dyn CWEventDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreWLAN_CWWiFiClient")]
    pub struct CWWiFiClient;

    #[cfg(feature = "CoreWLAN_CWWiFiClient")]
    unsafe impl ClassType for CWWiFiClient {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreWLAN_CWWiFiClient")]
unsafe impl NSObjectProtocol for CWWiFiClient {}

extern_methods!(
    #[cfg(feature = "CoreWLAN_CWWiFiClient")]
    unsafe impl CWWiFiClient {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AnyObject>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other sharedWiFiClient)]
        pub unsafe fn sharedWiFiClient() -> Id<CWWiFiClient>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CoreWLAN_CWInterface")]
        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface(&self) -> Option<Id<CWInterface>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[CWWiFiClient interfaceNames] instead"]
        #[method_id(@__retain_semantics Other interfaceNames)]
        pub unsafe fn interfaceNames_class() -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "CoreWLAN_CWInterface", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other interfaceWithName:)]
        pub unsafe fn interfaceWithName(
            &self,
            interface_name: Option<&NSString>,
        ) -> Option<Id<CWInterface>>;

        #[cfg(all(feature = "CoreWLAN_CWInterface", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other interfaces)]
        pub unsafe fn interfaces(&self) -> Option<Id<NSArray<CWInterface>>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(startMonitoringEventWithType:error:_)]
        pub unsafe fn startMonitoringEventWithType_error(
            &self,
            r#type: CWEventType,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(stopMonitoringEventWithType:error:_)]
        pub unsafe fn stopMonitoringEventWithType_error(
            &self,
            r#type: CWEventType,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(stopMonitoringAllEventsAndReturnError:_)]
        pub unsafe fn stopMonitoringAllEventsAndReturnError(&self) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreWLAN_CWWiFiClient")]
    unsafe impl CWWiFiClient {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
