use std::ffi::CString;
use std::ptr::null_mut;

pub struct GDALVectorTranslateOptions {
        _options: Vec<CString>,
        pub gdal_opts: *mut gdal_sys::GDALVectorTranslateOptions,
}

impl GDALVectorTranslateOptions {
        pub fn new(options: &[&str]) -> Self {
                let options: Vec<CString> =
                        options.iter().map(|x| CString::new(*x).unwrap()).collect();
                let mut ref_opts: Vec<*mut i8> =
                        options.iter().map(|x| x.as_ptr() as *mut _).collect();
                ref_opts.push(null_mut());
                let gdal_opts = unsafe {
                        gdal_sys::GDALVectorTranslateOptionsNew(ref_opts.as_mut_ptr(), null_mut())
                };
                Self {
                        _options: options,
                        gdal_opts,
                }
        }
}

impl Drop for GDALVectorTranslateOptions {
        fn drop(&mut self) {
                unsafe {
                        gdal_sys::GDALVectorTranslateOptionsFree(self.gdal_opts);
                }
        }
}
