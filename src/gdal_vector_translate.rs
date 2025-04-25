use std::ptr::{null, null_mut};

use std::ffi::{CString, c_int};

use gdal::Dataset;
use std::path::Path;

use super::gdal_vector_translate_options::GDALVectorTranslateOptions;

enum ZDataset<'a> {
        Dataset(&'a Dataset),
        Path(&'a Path),
}

#[derive(Clone, Debug)]
pub enum GDALError {
        Todo,
}

impl std::error::Error for GDALError {}

impl std::fmt::Display for GDALError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                        Self::Todo => write!(f, "TODO!"),
                }
        }
}

fn GDALVectorTranslate(
        src: &Dataset,
        dst: &ZDataset,
        options: &GDALVectorTranslateOptions,
) -> Result<(), GDALError> {
        let mut pb_usage_error: c_int = 0;
        match dst {
                ZDataset::Dataset(dataset) => unsafe {
                        gdal_sys::GDALVectorTranslate(
                                null(),
                                dataset.c_dataset(),
                                1,
                                &mut src.c_dataset(),
                                options.gdal_opts,
                                &mut pb_usage_error,
                        );
                },
                ZDataset::Path(path) => {
                        let path_ptr = CString::new(path.to_str().unwrap()).unwrap();
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
                }
        }
        if pb_usage_error != 0 {
                Err(GDALError::Todo)
        } else {
                Ok(())
        }
}
