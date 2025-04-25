use gdal::GdalOpenFlags;

pub trait Model {
        const FLAG: GdalOpenFlags;
}

pub struct Raster;
pub struct MultiDimRaster;
pub struct Vector;
pub struct GNM;

impl Model for Raster {
        const FLAG: GdalOpenFlags = GdalOpenFlags::GDAL_OF_RASTER;
}
impl Model for MultiDimRaster {
        const FLAG: GdalOpenFlags = GdalOpenFlags::GDAL_OF_MULTIDIM_RASTER;
}
impl Model for Vector {
        const FLAG: GdalOpenFlags = GdalOpenFlags::GDAL_OF_VECTOR;
}
impl Model for GNM {
        const FLAG: GdalOpenFlags = GdalOpenFlags::GDAL_OF_GNM;
}
