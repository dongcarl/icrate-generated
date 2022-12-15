//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXMLDTD;

    unsafe impl ClassType for NSXMLDTD {
        #[inherits(NSObject)]
        type Super = NSXMLNode;
    }
);

extern_methods!(
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:options:error:)]
        pub unsafe fn initWithData_options_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, publicID: Option<&NSString>);

        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, systemID: Option<&NSString>);

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        #[method_id(@__retain_semantics Other entityDeclarationForName:)]
        pub unsafe fn entityDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;

        #[method_id(@__retain_semantics Other notationDeclarationForName:)]
        pub unsafe fn notationDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;

        #[method_id(@__retain_semantics Other elementDeclarationForName:)]
        pub unsafe fn elementDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;

        #[method_id(@__retain_semantics Other attributeDeclarationForName:elementName:)]
        pub unsafe fn attributeDeclarationForName_elementName(
            &self,
            name: &NSString,
            elementName: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;

        #[method_id(@__retain_semantics Other predefinedEntityDeclarationForName:)]
        pub unsafe fn predefinedEntityDeclarationForName(
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclasses
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;
    }
);
