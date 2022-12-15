//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCandidateListTouchBarItem<
        CandidateType: Message = Object,
        CandidateTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (CandidateType, CandidateTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<CandidateType: Message, CandidateTypeOwnership: Ownership> ClassType
        for NSCandidateListTouchBarItem<CandidateType, CandidateTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    unsafe impl<CandidateType: Message, CandidateTypeOwnership: Ownership>
        NSCandidateListTouchBarItem<CandidateType, CandidateTypeOwnership>
    {
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<TodoProtocols, Shared>>;

        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&TodoProtocols>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSCandidateListTouchBarItemDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSCandidateListTouchBarItemDelegate>);

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        #[method(allowsCollapsing)]
        pub unsafe fn allowsCollapsing(&self) -> bool;

        #[method(setAllowsCollapsing:)]
        pub unsafe fn setAllowsCollapsing(&self, allowsCollapsing: bool);

        #[method(isCandidateListVisible)]
        pub unsafe fn isCandidateListVisible(&self) -> bool;

        #[method(updateWithInsertionPointVisibility:)]
        pub unsafe fn updateWithInsertionPointVisibility(&self, isVisible: bool);

        #[method(allowsTextInputContextCandidates)]
        pub unsafe fn allowsTextInputContextCandidates(&self) -> bool;

        #[method(setAllowsTextInputContextCandidates:)]
        pub unsafe fn setAllowsTextInputContextCandidates(
            &self,
            allowsTextInputContextCandidates: bool,
        );

        #[method(attributedStringForCandidate)]
        pub unsafe fn attributedStringForCandidate(
            &self,
        ) -> *mut Block<(NonNull<CandidateType>, NSInteger), NonNull<NSAttributedString>>;

        #[method(setAttributedStringForCandidate:)]
        pub unsafe fn setAttributedStringForCandidate(
            &self,
            attributedStringForCandidate: Option<
                &Block<(NonNull<CandidateType>, NSInteger), NonNull<NSAttributedString>>,
            >,
        );

        #[method_id(@__retain_semantics Other candidates)]
        pub unsafe fn candidates(&self) -> Id<NSArray<CandidateType>, Shared>;

        #[method(setCandidates:forSelectedRange:inString:)]
        pub unsafe fn setCandidates_forSelectedRange_inString(
            &self,
            candidates: &NSArray<CandidateType>,
            selectedRange: NSRange,
            originalString: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl<CandidateType: Message, CandidateTypeOwnership: Ownership>
        NSCandidateListTouchBarItem<CandidateType, CandidateTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);

extern_protocol!(
    pub struct NSCandidateListTouchBarItemDelegate;

    unsafe impl ProtocolType for NSCandidateListTouchBarItemDelegate {
        #[optional]
        #[method(candidateListTouchBarItem:beginSelectingCandidateAtIndex:)]
        pub unsafe fn candidateListTouchBarItem_beginSelectingCandidateAtIndex(
            &self,
            anItem: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[optional]
        #[method(candidateListTouchBarItem:changeSelectionFromCandidateAtIndex:toIndex:)]
        pub unsafe fn candidateListTouchBarItem_changeSelectionFromCandidateAtIndex_toIndex(
            &self,
            anItem: &NSCandidateListTouchBarItem,
            previousIndex: NSInteger,
            index: NSInteger,
        );

        #[optional]
        #[method(candidateListTouchBarItem:endSelectingCandidateAtIndex:)]
        pub unsafe fn candidateListTouchBarItem_endSelectingCandidateAtIndex(
            &self,
            anItem: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[optional]
        #[method(candidateListTouchBarItem:changedCandidateListVisibility:)]
        pub unsafe fn candidateListTouchBarItem_changedCandidateListVisibility(
            &self,
            anItem: &NSCandidateListTouchBarItem,
            isVisible: bool,
        );
    }
);

extern_methods!(
    /// NSCandidateListTouchBarItem
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(
            &self,
        ) -> Option<Id<NSCandidateListTouchBarItem, Shared>>;
    }
);

extern_static!(NSTouchBarItemIdentifierCandidateList: &'static NSTouchBarItemIdentifier);
