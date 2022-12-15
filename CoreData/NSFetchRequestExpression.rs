//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSFetchRequestExpressionType: NSExpressionType = 50);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchRequestExpression;

    unsafe impl ClassType for NSFetchRequestExpression {
        #[inherits(NSObject)]
        type Super = NSExpression;
    }
);

extern_methods!(
    unsafe impl NSFetchRequestExpression {
        #[method_id(@__retain_semantics Other expressionForFetch:context:countOnly:)]
        pub unsafe fn expressionForFetch_context_countOnly(
            fetch: &NSExpression,
            context: &NSExpression,
            countFlag: bool,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other requestExpression)]
        pub unsafe fn requestExpression(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other contextExpression)]
        pub unsafe fn contextExpression(&self) -> Id<NSExpression, Shared>;

        #[method(isCountOnlyRequest)]
        pub unsafe fn isCountOnlyRequest(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl NSFetchRequestExpression {
        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Option<Allocated<Self>>,
            type_: NSExpressionType,
        ) -> Id<Self, Shared>;
    }
);
