//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLIOCompressionStatus {
        MTLIOCompressionStatusComplete = 0,
        MTLIOCompressionStatusError = 1,
    }
);

pub type MTLIOCompressionContext = *mut c_void;

extern_fn!(
    pub unsafe fn MTLIOCompressionContextDefaultChunkSize() -> usize;
);

extern_fn!(
    pub unsafe fn MTLIOCreateCompressionContext(
        path: NonNull<c_char>,
        r#type: MTLIOCompressionMethod,
        chunk_size: usize,
    ) -> MTLIOCompressionContext;
);

extern_fn!(
    pub unsafe fn MTLIOCompressionContextAppendData(
        context: MTLIOCompressionContext,
        data: NonNull<c_void>,
        size: usize,
    );
);

extern_fn!(
    pub unsafe fn MTLIOFlushAndDestroyCompressionContext(
        context: MTLIOCompressionContext,
    ) -> MTLIOCompressionStatus;
);