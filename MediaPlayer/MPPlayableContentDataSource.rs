//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_protocol!(
    #[deprecated = "Use CarPlay framework"]
    pub unsafe trait MPPlayableContentDataSource: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSIndexPath"))]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(beginLoadingChildItemsAtIndexPath:completionHandler:)]
        unsafe fn beginLoadingChildItemsAtIndexPath_completionHandler(
            &self,
            index_path: &NSIndexPath,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(childItemsDisplayPlaybackProgressAtIndexPath:)]
        unsafe fn childItemsDisplayPlaybackProgressAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "MediaPlayer_MPContentItem"
        ))]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(contentItemForIdentifier:completionHandler:)]
        unsafe fn contentItemForIdentifier_completionHandler(
            &self,
            identifier: &NSString,
            completion_handler: &Block<(*mut MPContentItem, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[deprecated = "Use CarPlay framework"]
        #[method(numberOfChildItemsAtIndexPath:)]
        unsafe fn numberOfChildItemsAtIndexPath(&self, index_path: &NSIndexPath) -> NSInteger;

        #[cfg(all(
            feature = "Foundation_NSIndexPath",
            feature = "MediaPlayer_MPContentItem"
        ))]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other contentItemAtIndexPath:)]
        unsafe fn contentItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Id<MPContentItem>>;
    }

    unsafe impl ProtocolType for dyn MPPlayableContentDataSource {}
);
