//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSDetailedErrorsKey: &'static NSString);

extern_static!(NSValidationObjectErrorKey: &'static NSString);

extern_static!(NSValidationKeyErrorKey: &'static NSString);

extern_static!(NSValidationPredicateErrorKey: &'static NSString);

extern_static!(NSValidationValueErrorKey: &'static NSString);

extern_static!(NSAffectedStoresErrorKey: &'static NSString);

extern_static!(NSAffectedObjectsErrorKey: &'static NSString);

extern_static!(NSPersistentStoreSaveConflictsErrorKey: &'static NSString);

extern_static!(NSSQLiteErrorDomain: &'static NSString);

extern_enum!(
    #[underlying(NSInteger)]
    pub enum __anonymous__ {
        NSManagedObjectValidationError = 1550,
        NSManagedObjectConstraintValidationError = 1551,
        NSValidationMultipleErrorsError = 1560,
        NSValidationMissingMandatoryPropertyError = 1570,
        NSValidationRelationshipLacksMinimumCountError = 1580,
        NSValidationRelationshipExceedsMaximumCountError = 1590,
        NSValidationRelationshipDeniedDeleteError = 1600,
        NSValidationNumberTooLargeError = 1610,
        NSValidationNumberTooSmallError = 1620,
        NSValidationDateTooLateError = 1630,
        NSValidationDateTooSoonError = 1640,
        NSValidationInvalidDateError = 1650,
        NSValidationStringTooLongError = 1660,
        NSValidationStringTooShortError = 1670,
        NSValidationStringPatternMatchingError = 1680,
        NSValidationInvalidURIError = 1690,
        NSManagedObjectContextLockingError = 132000,
        NSPersistentStoreCoordinatorLockingError = 132010,
        NSManagedObjectReferentialIntegrityError = 133000,
        NSManagedObjectExternalRelationshipError = 133010,
        NSManagedObjectMergeError = 133020,
        NSManagedObjectConstraintMergeError = 133021,
        NSPersistentStoreInvalidTypeError = 134000,
        NSPersistentStoreTypeMismatchError = 134010,
        NSPersistentStoreIncompatibleSchemaError = 134020,
        NSPersistentStoreSaveError = 134030,
        NSPersistentStoreIncompleteSaveError = 134040,
        NSPersistentStoreSaveConflictsError = 134050,
        NSCoreDataError = 134060,
        NSPersistentStoreOperationError = 134070,
        NSPersistentStoreOpenError = 134080,
        NSPersistentStoreTimeoutError = 134090,
        NSPersistentStoreUnsupportedRequestTypeError = 134091,
        NSPersistentStoreIncompatibleVersionHashError = 134100,
        NSMigrationError = 134110,
        NSMigrationConstraintViolationError = 134111,
        NSMigrationCancelledError = 134120,
        NSMigrationMissingSourceModelError = 134130,
        NSMigrationMissingMappingModelError = 134140,
        NSMigrationManagerSourceStoreError = 134150,
        NSMigrationManagerDestinationStoreError = 134160,
        NSEntityMigrationPolicyError = 134170,
        NSSQLiteError = 134180,
        NSInferredMappingModelError = 134190,
        NSExternalRecordImportError = 134200,
        NSPersistentHistoryTokenExpiredError = 134301,
        NSManagedObjectModelReferenceNotFoundError = 134504,
        NSStagedMigrationFrameworkVersionMismatchError = 134505,
        NSStagedMigrationBackwardMigrationError = 134506,
    }
);
