//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum _SFTranscriptionResultAttributeOptions {
        _SFTranscriptionResultAttributeOptionsConfidence = 1 << 0,
        _SFTranscriptionResultAttributeOptionsCmTime = 1 << 1,
    }
);
