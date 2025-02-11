//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundAssets::*;
use crate::Foundation::*;

extern_static!(BAErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum BAErrorCode {
        BAErrorCodeDownloadInvalid = 0,
        BAErrorCodeCallFromExtensionNotAllowed = 50,
        BAErrorCodeCallFromInactiveProcessNotAllowed = 51,
        BAErrorCodeCallerConnectionNotAccepted = 55,
        BAErrorCodeCallerConnectionInvalid = 56,
        BAErrorCodeDownloadAlreadyScheduled = 100,
        BAErrorCodeDownloadNotScheduled = 101,
        BAErrorCodeDownloadFailedToStart = 102,
        BAErrorCodeDownloadAlreadyFailed = 103,
        BAErrorCodeDownloadEssentialDownloadNotPermitted = 109,
        BAErrorCodeDownloadBackgroundActivityProhibited = 111,
        BAErrorCodeDownloadWouldExceedAllowance = 112,
        BAErrorCodeSessionDownloadDisallowedByDomain = 202,
        BAErrorCodeSessionDownloadDisallowedByAllowance = 203,
        BAErrorCodeSessionDownloadAllowanceExceeded = 204,
        BAErrorCodeSessionDownloadNotPermittedBeforeAppLaunch = 206,
    }
);
