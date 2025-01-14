// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ExceptionHandling` framework

#[cfg_attr(
    feature = "apple",
    link(name = "ExceptionHandling", kind = "framework")
)]
extern "C" {}

#[path = "ExceptionHandlingDefines.rs"]
mod __ExceptionHandlingDefines;
#[path = "NSExceptionHandler.rs"]
mod __NSExceptionHandler;

#[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSExceptionHandler;
pub use self::__NSExceptionHandler::NSExceptionHandlerResume;
pub use self::__NSExceptionHandler::NSStackTraceKey;
pub use self::__NSExceptionHandler::NSUncaughtRuntimeErrorException;
pub use self::__NSExceptionHandler::NSUncaughtSystemExceptionException;
pub use self::__NSExceptionHandler::{
    NSHandleOtherExceptionMask, NSHandleTopLevelExceptionMask, NSHandleUncaughtExceptionMask,
    NSHandleUncaughtRuntimeErrorMask, NSHandleUncaughtSystemExceptionMask, NSLogOtherExceptionMask,
    NSLogTopLevelExceptionMask, NSLogUncaughtExceptionMask, NSLogUncaughtRuntimeErrorMask,
    NSLogUncaughtSystemExceptionMask,
};
pub use self::__NSExceptionHandler::{
    NSHangOnOtherExceptionMask, NSHangOnTopLevelExceptionMask, NSHangOnUncaughtExceptionMask,
    NSHangOnUncaughtRuntimeErrorMask, NSHangOnUncaughtSystemExceptionMask,
};
