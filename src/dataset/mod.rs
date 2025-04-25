pub mod cache;
pub mod dataset_builder;
pub mod mode;
pub mod models;

use std::marker::PhantomData;

use core::ffi::c_void;

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
        fn layer(
                &self,
                idx: usize,
        ) -> std::result::Result<gdal::vector::Layer<'_>, gdal::errors::GdalError> {
                self.ds.layer(idx)
        }
}
