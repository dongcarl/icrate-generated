//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Hash)]
    pub struct NSValue;

    unsafe impl ClassType for NSValue {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSValue {
        #[method(getValue:size:)]
        pub unsafe fn getValue_size(&self, value: NonNull<c_void>, size: NSUInteger);

        #[method(objCType)]
        pub fn objCType(&self) -> NonNull<c_char>;

        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Option<Allocated<Self>>,
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSValueCreation
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithBytes:objCType:)]
        pub unsafe fn valueWithBytes_objCType(
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<NSValue, Shared>;

        #[method_id(@__retain_semantics Other value:withObjCType:)]
        pub unsafe fn value_withObjCType(
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<NSValue, Shared>;
    }
);

extern_methods!(
    /// NSValueExtensionMethods
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithNonretainedObject:)]
        pub unsafe fn valueWithNonretainedObject(anObject: Option<&Object>) -> Id<NSValue, Shared>;

        #[method_id(@__retain_semantics Other nonretainedObjectValue)]
        pub unsafe fn nonretainedObjectValue(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other valueWithPointer:)]
        pub unsafe fn valueWithPointer(pointer: *mut c_void) -> Id<NSValue, Shared>;

        #[method(pointerValue)]
        pub unsafe fn pointerValue(&self) -> *mut c_void;

        #[method(isEqualToValue:)]
        pub fn isEqualToValue(&self, value: &NSValue) -> bool;
    }
);

extern_class!(
    #[derive(Hash)]
    pub struct NSNumber;

    unsafe impl ClassType for NSNumber {
        #[inherits(NSObject)]
        type Super = NSValue;
    }
);

extern_methods!(
    unsafe impl NSNumber {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithChar:)]
        pub fn initWithChar(this: Option<Allocated<Self>>, value: c_char) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedChar:)]
        pub fn initWithUnsignedChar(
            this: Option<Allocated<Self>>,
            value: c_uchar,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithShort:)]
        pub fn initWithShort(this: Option<Allocated<Self>>, value: c_short)
            -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedShort:)]
        pub fn initWithUnsignedShort(
            this: Option<Allocated<Self>>,
            value: c_ushort,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithInt:)]
        pub fn initWithInt(this: Option<Allocated<Self>>, value: c_int) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedInt:)]
        pub fn initWithUnsignedInt(
            this: Option<Allocated<Self>>,
            value: c_uint,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithLong:)]
        pub fn initWithLong(this: Option<Allocated<Self>>, value: c_long) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedLong:)]
        pub fn initWithUnsignedLong(
            this: Option<Allocated<Self>>,
            value: c_ulong,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithLongLong:)]
        pub fn initWithLongLong(
            this: Option<Allocated<Self>>,
            value: c_longlong,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedLongLong:)]
        pub fn initWithUnsignedLongLong(
            this: Option<Allocated<Self>>,
            value: c_ulonglong,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithFloat:)]
        pub fn initWithFloat(this: Option<Allocated<Self>>, value: c_float)
            -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithDouble:)]
        pub fn initWithDouble(
            this: Option<Allocated<Self>>,
            value: c_double,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithBool:)]
        pub fn initWithBool(this: Option<Allocated<Self>>, value: bool) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithInteger:)]
        pub fn initWithInteger(
            this: Option<Allocated<Self>>,
            value: NSInteger,
        ) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Init initWithUnsignedInteger:)]
        pub fn initWithUnsignedInteger(
            this: Option<Allocated<Self>>,
            value: NSUInteger,
        ) -> Id<NSNumber, Shared>;

        #[method(charValue)]
        pub fn charValue(&self) -> c_char;

        #[method(unsignedCharValue)]
        pub fn unsignedCharValue(&self) -> c_uchar;

        #[method(shortValue)]
        pub fn shortValue(&self) -> c_short;

        #[method(unsignedShortValue)]
        pub fn unsignedShortValue(&self) -> c_ushort;

        #[method(intValue)]
        pub fn intValue(&self) -> c_int;

        #[method(unsignedIntValue)]
        pub fn unsignedIntValue(&self) -> c_uint;

        #[method(longValue)]
        pub fn longValue(&self) -> c_long;

        #[method(unsignedLongValue)]
        pub fn unsignedLongValue(&self) -> c_ulong;

        #[method(longLongValue)]
        pub fn longLongValue(&self) -> c_longlong;

        #[method(unsignedLongLongValue)]
        pub fn unsignedLongLongValue(&self) -> c_ulonglong;

        #[method(floatValue)]
        pub fn floatValue(&self) -> c_float;

        #[method(doubleValue)]
        pub fn doubleValue(&self) -> c_double;

        #[method(boolValue)]
        pub fn boolValue(&self) -> bool;

        #[method(integerValue)]
        pub fn integerValue(&self) -> NSInteger;

        #[method(unsignedIntegerValue)]
        pub fn unsignedIntegerValue(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other stringValue)]
        pub fn stringValue(&self) -> Id<NSString, Shared>;

        #[method(compare:)]
        pub fn compare(&self, otherNumber: &NSNumber) -> NSComparisonResult;

        #[method(isEqualToNumber:)]
        pub fn isEqualToNumber(&self, number: &NSNumber) -> bool;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl NSNumber {
        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Option<Allocated<Self>>,
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSNumberCreation
    unsafe impl NSNumber {
        #[method_id(@__retain_semantics Other numberWithChar:)]
        pub fn numberWithChar(value: c_char) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedChar:)]
        pub fn numberWithUnsignedChar(value: c_uchar) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithShort:)]
        pub fn numberWithShort(value: c_short) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedShort:)]
        pub fn numberWithUnsignedShort(value: c_ushort) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithInt:)]
        pub fn numberWithInt(value: c_int) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedInt:)]
        pub fn numberWithUnsignedInt(value: c_uint) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithLong:)]
        pub fn numberWithLong(value: c_long) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedLong:)]
        pub fn numberWithUnsignedLong(value: c_ulong) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithLongLong:)]
        pub fn numberWithLongLong(value: c_longlong) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedLongLong:)]
        pub fn numberWithUnsignedLongLong(value: c_ulonglong) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithFloat:)]
        pub fn numberWithFloat(value: c_float) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithDouble:)]
        pub fn numberWithDouble(value: c_double) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithBool:)]
        pub fn numberWithBool(value: bool) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithInteger:)]
        pub fn numberWithInteger(value: NSInteger) -> Id<NSNumber, Shared>;

        #[method_id(@__retain_semantics Other numberWithUnsignedInteger:)]
        pub fn numberWithUnsignedInteger(value: NSUInteger) -> Id<NSNumber, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSValue {
        #[method(getValue:)]
        pub unsafe fn getValue(&self, value: NonNull<c_void>);
    }
);
