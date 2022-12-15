//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPopoverTouchBarItem;

    unsafe impl ClassType for NSPopoverTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics Other popoverTouchBar)]
        pub unsafe fn popoverTouchBar(&self) -> Id<NSTouchBar, Shared>;

        #[method(setPopoverTouchBar:)]
        pub unsafe fn setPopoverTouchBar(&self, popoverTouchBar: &NSTouchBar);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);

        #[method_id(@__retain_semantics Other collapsedRepresentation)]
        pub unsafe fn collapsedRepresentation(&self) -> Id<NSView, Shared>;

        #[method(setCollapsedRepresentation:)]
        pub unsafe fn setCollapsedRepresentation(&self, collapsedRepresentation: &NSView);

        #[method_id(@__retain_semantics Other collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsedRepresentationImage: Option<&NSImage>,
        );

        #[method_id(@__retain_semantics Other collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Id<NSString, Shared>;

        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsedRepresentationLabel: &NSString,
        );

        #[method_id(@__retain_semantics Other pressAndHoldTouchBar)]
        pub unsafe fn pressAndHoldTouchBar(&self) -> Option<Id<NSTouchBar, Shared>>;

        #[method(setPressAndHoldTouchBar:)]
        pub unsafe fn setPressAndHoldTouchBar(&self, pressAndHoldTouchBar: Option<&NSTouchBar>);

        #[method(showsCloseButton)]
        pub unsafe fn showsCloseButton(&self) -> bool;

        #[method(setShowsCloseButton:)]
        pub unsafe fn setShowsCloseButton(&self, showsCloseButton: bool);

        #[method(showPopover:)]
        pub unsafe fn showPopover(&self, sender: Option<&Object>);

        #[method(dismissPopover:)]
        pub unsafe fn dismissPopover(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other makeStandardActivatePopoverGestureRecognizer)]
        pub unsafe fn makeStandardActivatePopoverGestureRecognizer(
            &self,
        ) -> Id<NSGestureRecognizer, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
