use std::marker::PhantomData;
use std::path::Path;

use gdal::GdalOpenFlags;

use super::cache::Cache;
use super::mode::Mode;
use super::models;

pub struct Dataset<T> {
        ds: gdal::Dataset,
        t: PhantomData<T>,
}

impl<T> Dataset<T> {
        fn new(ds: gdal::Dataset) -> Dataset<T> {
                Dataset { ds, t: PhantomData }
        }
}

type AllowedDrivers<'a> = Option<&'a [&'a str]>;
type OpenOptions<'a> = Option<&'a [&'a str]>;
type SiblingFiles<'a> = Option<&'a [&'a str]>;

pub struct DatasetBuilder<'a, T: models::Model, P: AsRef<Path>> {
        pub path: P,
        pub mode: Mode,
        pub cache: Cache,
        pub allowed_drivers: AllowedDrivers<'a>,
        pub open_options: OpenOptions<'a>,
        pub sibling_files: SiblingFiles<'a>,
        pub model: PhantomData<T>,
}

impl<'a, T: models::Model, P: AsRef<Path>> DatasetBuilder<'a, T, P> {
        fn new(path: P, mode: Mode) -> Self {
                DatasetBuilder {
                        path,
                        mode,
                        cache: Cache::GDALDefault,
                        allowed_drivers: None,
                        open_options: None,
                        sibling_files: None,
                        model: PhantomData,
                }
        }
        fn set_cache(mut self, cache: Cache) -> Self {
                self.cache = cache;
                self
        }
        fn set_allowed_drivers(mut self, allowed_drivers: AllowedDrivers<'a>) -> Self {
                self.allowed_drivers = allowed_drivers;
                self
        }
        fn set_open_options(mut self, open_options: OpenOptions<'a>) -> Self {
                self.open_options = open_options;
                self
        }
        fn set_sibling_files(mut self, sibling_files: SiblingFiles<'a>) -> Self {
                self.sibling_files = sibling_files;
                self
        }
        fn open(self) -> Result<Dataset<T>, Box<dyn std::error::Error>> {
                let mut open_flags = T::FLAG;
                open_flags |= self.mode.into();
                open_flags |= self.cache.into();
                let opts = gdal::DatasetOptions {
                        open_flags,
                        allowed_drivers: self.allowed_drivers,
                        open_options: self.open_options,
                        sibling_files: self.sibling_files,
                };
                let ds = gdal::Dataset::open_ex(self.path, opts)?;
                Ok(Dataset::new(ds))
        }
}
