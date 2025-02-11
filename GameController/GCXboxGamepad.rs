//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCXboxGamepad")]
    pub struct GCXboxGamepad;

    #[cfg(feature = "GameController_GCXboxGamepad")]
    unsafe impl ClassType for GCXboxGamepad {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCXboxGamepad")]
unsafe impl NSObjectProtocol for GCXboxGamepad {}

extern_methods!(
    #[cfg(feature = "GameController_GCXboxGamepad")]
    unsafe impl GCXboxGamepad {
        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other paddleButton1)]
        pub unsafe fn paddleButton1(&self) -> Option<Id<GCControllerButtonInput>>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other paddleButton2)]
        pub unsafe fn paddleButton2(&self) -> Option<Id<GCControllerButtonInput>>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other paddleButton3)]
        pub unsafe fn paddleButton3(&self) -> Option<Id<GCControllerButtonInput>>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other paddleButton4)]
        pub unsafe fn paddleButton4(&self) -> Option<Id<GCControllerButtonInput>>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonShare)]
        pub unsafe fn buttonShare(&self) -> Option<Id<GCControllerButtonInput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCXboxGamepad")]
    unsafe impl GCXboxGamepad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
