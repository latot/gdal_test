pub mod cache;
pub mod dataset_builder;
pub mod mode;
pub mod models;

use std::marker::PhantomData;

use core::ffi::c_void;
use gdal::errors::Result;

pub struct Dataset<T> {
        ds: gdal::Dataset,
        t: PhantomData<T>,
}

impl<T> Dataset<T> {
        fn new(ds: gdal::Dataset) -> Dataset<T> {
                Dataset { ds, t: PhantomData }
        }
        fn c_dataset(&self) -> *mut c_void {
                self.ds.c_dataset()
        }
}

impl Dataset<models::Vector> {
        fn layer(&self, idx: usize) -> Result<gdal::vector::Layer<'_>> {
                self.ds.layer(idx)
        }
}

impl Dataset<models::Raster> {
        fn raster_count(&self) -> usize {
                self.ds.raster_count()
        }
        fn geo_transform(&self) -> Result<gdal::GeoTransform> {
                self.ds.geo_transform()
        }
        fn gcp_projection(&self) -> Option<String> {
                self.ds.gcp_projection()
        }
}
