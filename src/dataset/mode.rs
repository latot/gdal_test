use gdal::GdalOpenFlags;

pub enum Mode {
        ReadOnly,
        Update,
}

impl From<Mode> for GdalOpenFlags {
        fn from(value: Mode) -> Self {
                match value {
                        Mode::ReadOnly => GdalOpenFlags::GDAL_OF_READONLY,
                        Mode::Update => GdalOpenFlags::GDAL_OF_UPDATE,
                }
        }
}

impl From<&Mode> for GdalOpenFlags {
        fn from(value: &Mode) -> Self {
                match value {
                        Mode::ReadOnly => GdalOpenFlags::GDAL_OF_READONLY,
                        Mode::Update => GdalOpenFlags::GDAL_OF_UPDATE,
                }
        }
}
