use std::ptr::{null, null_mut};

use std::ffi::{CString, c_int};

use gdal::Dataset;
use std::path::Path;

use super::gdal_vector_translate::GDALError;
use super::gdal_vector_translate_options::GDALVectorTranslateOptions;
use std::marker::PhantomData;

struct GDALVectorTranslate<T> {
        dst: PhantomData<T>,
}

impl GDALVectorTranslate<&Dataset> {
        pub fn run(
                src: &Dataset,
                dst: &Dataset,
                options: &GDALVectorTranslateOptions,
        ) -> Result<(), GDALError> {
                let mut pb_usage_error: c_int = 0;
                unsafe {
                        gdal_sys::GDALVectorTranslate(
                                null(),
                                dst.c_dataset(),
                                1,
                                &mut src.c_dataset(),
                                options.gdal_opts,
                                &mut pb_usage_error,
                        );
                }
                if pb_usage_error != 0 {
                        Err(GDALError::Todo)
                } else {
                        Ok(())
                }
        }
}

impl GDALVectorTranslate<&Path> {
        pub fn run(
                src: &Dataset,
                dst: &Path,
                options: &GDALVectorTranslateOptions,
        ) -> Result<(), GDALError> {
                let mut pb_usage_error: c_int = 0;
                let path_ptr = CString::new(dst.to_str().unwrap()).unwrap();
                unsafe {
                        gdal_sys::GDALVectorTranslate(
                                path_ptr.as_ptr(),
                                null_mut(),
                                1,
                                &mut src.c_dataset(),
                                options.gdal_opts,
                                &mut pb_usage_error,
                        );
                }
                if pb_usage_error != 0 {
                        Err(GDALError::Todo)
                } else {
                        Ok(())
                }
        }
}
