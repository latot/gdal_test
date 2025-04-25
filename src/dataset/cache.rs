use gdal::GdalOpenFlags;
pub enum Cache {
        // Allow GDAL choose which one
        GDALDefault,
        Array,
        Hashset,
}

impl From<Cache> for GdalOpenFlags {
        fn from(value: Cache) -> Self {
                match value {
                        Cache::Array => GdalOpenFlags::GDAL_OF_ARRAY_BLOCK_ACCESS,
                        Cache::GDALDefault => GdalOpenFlags::GDAL_OF_DEFAULT_BLOCK_ACCESS,
                        Cache::Hashset => GdalOpenFlags::GDAL_OF_HASHSET_BLOCK_ACCESS,
                }
        }
}

impl From<&Cache> for GdalOpenFlags {
        fn from(value: &Cache) -> Self {
                match value {
                        Cache::Array => GdalOpenFlags::GDAL_OF_ARRAY_BLOCK_ACCESS,
                        Cache::GDALDefault => GdalOpenFlags::GDAL_OF_DEFAULT_BLOCK_ACCESS,
                        Cache::Hashset => GdalOpenFlags::GDAL_OF_HASHSET_BLOCK_ACCESS,
                }
        }
}
