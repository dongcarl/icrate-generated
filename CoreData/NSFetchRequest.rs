//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFetchRequestResultType {
        NSManagedObjectResultType = 0x00,
        NSManagedObjectIDResultType = 0x01,
        NSDictionaryResultType = 0x02,
        NSCountResultType = 0x04,
    }
);

extern_protocol!(
    pub struct NSFetchRequestResult;

    unsafe impl ProtocolType for NSFetchRequestResult {}
);

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "Foundation_NSNumber")]
    unsafe impl Foundation::NSNumber {}
);

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "Foundation_NSDictionary")]
    unsafe impl Foundation::NSDictionary {}
);

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "CoreData_NSManagedObject")]
    unsafe impl CoreData::NSManagedObject {}
);

extern_methods!(
    /// NSFetchedResultSupport
    #[cfg(feature = "CoreData_NSManagedObjectID")]
    unsafe impl CoreData::NSManagedObjectID {}
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchRequest<ResultType: Message = Object, ResultTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ResultType, ResultTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership> ClassType
        for NSFetchRequest<ResultType, ResultTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = CoreData::NSPersistentStoreRequest;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchRequest")]
    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership>
        NSFetchRequest<ResultType, ResultTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fetchRequestWithEntityName:)]
        pub unsafe fn fetchRequestWithEntityName(
            entityName: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithEntityName:)]
        pub unsafe fn initWithEntityName(
            this: Option<Allocated<Self>>,
            entityName: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other execute:_)]
        pub unsafe fn execute(
            &self,
        ) -> Result<Id<Foundation::NSArray<ResultType>, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<CoreData::NSEntityDescription, Shared>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method(setEntity:)]
        pub unsafe fn setEntity(&self, entity: Option<&CoreData::NSEntityDescription>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<Foundation::NSPredicate, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&Foundation::NSPredicate>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSSortDescriptor>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sortDescriptors: Option<&Foundation::NSArray<Foundation::NSSortDescriptor>>,
        );

        #[method(fetchLimit)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        #[method(setFetchLimit:)]
        pub unsafe fn setFetchLimit(&self, fetchLimit: NSUInteger);

        #[cfg(all(feature = "CoreData_NSPersistentStore", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other affectedStores)]
        pub unsafe fn affectedStores(
            &self,
        ) -> Option<Id<Foundation::NSArray<CoreData::NSPersistentStore>, Shared>>;

        #[cfg(all(feature = "CoreData_NSPersistentStore", feature = "Foundation_NSArray"))]
        #[method(setAffectedStores:)]
        pub unsafe fn setAffectedStores(
            &self,
            affectedStores: Option<&Foundation::NSArray<CoreData::NSPersistentStore>>,
        );

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSFetchRequestResultType;

        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, resultType: NSFetchRequestResultType);

        #[method(includesSubentities)]
        pub unsafe fn includesSubentities(&self) -> bool;

        #[method(setIncludesSubentities:)]
        pub unsafe fn setIncludesSubentities(&self, includesSubentities: bool);

        #[method(includesPropertyValues)]
        pub unsafe fn includesPropertyValues(&self) -> bool;

        #[method(setIncludesPropertyValues:)]
        pub unsafe fn setIncludesPropertyValues(&self, includesPropertyValues: bool);

        #[method(returnsObjectsAsFaults)]
        pub unsafe fn returnsObjectsAsFaults(&self) -> bool;

        #[method(setReturnsObjectsAsFaults:)]
        pub unsafe fn setReturnsObjectsAsFaults(&self, returnsObjectsAsFaults: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other relationshipKeyPathsForPrefetching)]
        pub unsafe fn relationshipKeyPathsForPrefetching(
            &self,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setRelationshipKeyPathsForPrefetching:)]
        pub unsafe fn setRelationshipKeyPathsForPrefetching(
            &self,
            relationshipKeyPathsForPrefetching: Option<&Foundation::NSArray<Foundation::NSString>>,
        );

        #[method(includesPendingChanges)]
        pub unsafe fn includesPendingChanges(&self) -> bool;

        #[method(setIncludesPendingChanges:)]
        pub unsafe fn setIncludesPendingChanges(&self, includesPendingChanges: bool);

        #[method(returnsDistinctResults)]
        pub unsafe fn returnsDistinctResults(&self) -> bool;

        #[method(setReturnsDistinctResults:)]
        pub unsafe fn setReturnsDistinctResults(&self, returnsDistinctResults: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other propertiesToFetch)]
        pub unsafe fn propertiesToFetch(&self) -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPropertiesToFetch:)]
        pub unsafe fn setPropertiesToFetch(&self, propertiesToFetch: Option<&Foundation::NSArray>);

        #[method(fetchOffset)]
        pub unsafe fn fetchOffset(&self) -> NSUInteger;

        #[method(setFetchOffset:)]
        pub unsafe fn setFetchOffset(&self, fetchOffset: NSUInteger);

        #[method(fetchBatchSize)]
        pub unsafe fn fetchBatchSize(&self) -> NSUInteger;

        #[method(setFetchBatchSize:)]
        pub unsafe fn setFetchBatchSize(&self, fetchBatchSize: NSUInteger);

        #[method(shouldRefreshRefetchedObjects)]
        pub unsafe fn shouldRefreshRefetchedObjects(&self) -> bool;

        #[method(setShouldRefreshRefetchedObjects:)]
        pub unsafe fn setShouldRefreshRefetchedObjects(&self, shouldRefreshRefetchedObjects: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other propertiesToGroupBy)]
        pub unsafe fn propertiesToGroupBy(&self) -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPropertiesToGroupBy:)]
        pub unsafe fn setPropertiesToGroupBy(
            &self,
            propertiesToGroupBy: Option<&Foundation::NSArray>,
        );

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other havingPredicate)]
        pub unsafe fn havingPredicate(&self) -> Option<Id<Foundation::NSPredicate, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setHavingPredicate:)]
        pub unsafe fn setHavingPredicate(&self, havingPredicate: Option<&Foundation::NSPredicate>);
    }
);

pub type NSPersistentStoreAsynchronousFetchResultCompletionBlock =
    *mut Block<(NonNull<CoreData::NSAsynchronousFetchResult>,), ()>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAsynchronousFetchRequest<
        ResultType: Message = Object,
        ResultTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ResultType, ResultTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership> ClassType
        for NSAsynchronousFetchRequest<ResultType, ResultTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = CoreData::NSPersistentStoreRequest;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership>
        NSAsynchronousFetchRequest<ResultType, ResultTypeOwnership>
    {
        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<CoreData::NSFetchRequest<ResultType>, Shared>;

        #[method(completionBlock)]
        pub unsafe fn completionBlock(
            &self,
        ) -> NSPersistentStoreAsynchronousFetchResultCompletionBlock;

        #[method(estimatedResultCount)]
        pub unsafe fn estimatedResultCount(&self) -> NSInteger;

        #[method(setEstimatedResultCount:)]
        pub unsafe fn setEstimatedResultCount(&self, estimatedResultCount: NSInteger);

        #[cfg(all(
            feature = "CoreData_NSAsynchronousFetchResult",
            feature = "CoreData_NSFetchRequest"
        ))]
        #[method_id(@__retain_semantics Init initWithFetchRequest:completionBlock:)]
        pub unsafe fn initWithFetchRequest_completionBlock(
            this: Option<Allocated<Self>>,
            request: &CoreData::NSFetchRequest<ResultType>,
            blk: Option<&Block<(NonNull<CoreData::NSAsynchronousFetchResult<ResultType>>,), ()>>,
        ) -> Id<Self, Shared>;
    }
);
