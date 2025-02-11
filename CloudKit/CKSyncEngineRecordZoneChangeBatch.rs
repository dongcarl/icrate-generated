//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
    pub struct CKSyncEngineRecordZoneChangeBatch;

    #[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
    unsafe impl ClassType for CKSyncEngineRecordZoneChangeBatch {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
unsafe impl Send for CKSyncEngineRecordZoneChangeBatch {}

#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
unsafe impl Sync for CKSyncEngineRecordZoneChangeBatch {}

#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
unsafe impl NSObjectProtocol for CKSyncEngineRecordZoneChangeBatch {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
    unsafe impl CKSyncEngineRecordZoneChangeBatch {
        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKSyncEnginePendingRecordZoneChange",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithPendingChanges:recordProvider:)]
        pub unsafe fn initWithPendingChanges_recordProvider(
            this: Allocated<Self>,
            pending_changes: &NSArray<CKSyncEnginePendingRecordZoneChange>,
            record_provider: &Block<(NonNull<CKRecordID>,), *mut CKRecord>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordsToSave:recordIDsToDelete:atomicByZone:)]
        pub unsafe fn initWithRecordsToSave_recordIDsToDelete_atomicByZone(
            this: Allocated<Self>,
            records_to_save: Option<&NSArray<CKRecord>>,
            record_i_ds_to_delete: Option<&NSArray<CKRecordID>>,
            atomic_by_zone: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordsToSave)]
        pub unsafe fn recordsToSave(&self) -> Id<NSArray<CKRecord>>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordIDsToDelete)]
        pub unsafe fn recordIDsToDelete(&self) -> Id<NSArray<CKRecordID>>;

        #[method(atomicByZone)]
        pub unsafe fn atomicByZone(&self) -> bool;

        #[method(setAtomicByZone:)]
        pub unsafe fn setAtomicByZone(&self, atomic_by_zone: bool);
    }
);
