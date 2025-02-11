//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    pub struct MKOverlayRenderer;

    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    unsafe impl ClassType for MKOverlayRenderer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKOverlayRenderer")]
unsafe impl NSObjectProtocol for MKOverlayRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    unsafe impl MKOverlayRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other overlay)]
        pub unsafe fn overlay(&self) -> Id<ProtocolObject<dyn MKOverlay>>;

        #[method(pointForMapPoint:)]
        pub unsafe fn pointForMapPoint(&self, map_point: MKMapPoint) -> CGPoint;

        #[method(mapPointForPoint:)]
        pub unsafe fn mapPointForPoint(&self, point: CGPoint) -> MKMapPoint;

        #[method(rectForMapRect:)]
        pub unsafe fn rectForMapRect(&self, map_rect: MKMapRect) -> CGRect;

        #[method(mapRectForRect:)]
        pub unsafe fn mapRectForRect(&self, rect: CGRect) -> MKMapRect;

        #[method(canDrawMapRect:zoomScale:)]
        pub unsafe fn canDrawMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        ) -> bool;

        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[method(setNeedsDisplayInMapRect:)]
        pub unsafe fn setNeedsDisplayInMapRect(&self, map_rect: MKMapRect);

        #[method(setNeedsDisplayInMapRect:zoomScale:)]
        pub unsafe fn setNeedsDisplayInMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        );

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(contentScaleFactor)]
        pub unsafe fn contentScaleFactor(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    unsafe impl MKOverlayRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_fn!(
    pub unsafe fn MKRoadWidthAtZoomScale(zoom_scale: MKZoomScale) -> CGFloat;
);
