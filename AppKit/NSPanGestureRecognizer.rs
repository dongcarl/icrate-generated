//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPanGestureRecognizer;

    unsafe impl ClassType for NSPanGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
    }
);

extern_methods!(
    unsafe impl NSPanGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, buttonMask: NSUInteger);

        #[method(translationInView:)]
        pub unsafe fn translationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[method(setTranslation:inView:)]
        pub unsafe fn setTranslation_inView(&self, translation: NSPoint, view: Option<&NSView>);

        #[method(velocityInView:)]
        pub unsafe fn velocityInView(&self, view: Option<&NSView>) -> NSPoint;

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, numberOfTouchesRequired: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl NSPanGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);
