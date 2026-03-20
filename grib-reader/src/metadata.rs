//! Edition-independent field metadata.

/// Common reference time representation for GRIB fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Edition-independent parameter identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameter {
    pub discipline: Option<u8>,
    pub category: Option<u8>,
    pub table_version: Option<u8>,
    pub number: u8,
    pub short_name: &'static str,
    pub description: &'static str,
}

impl Parameter {
    pub fn new_grib1(
        table_version: u8,
        number: u8,
        short_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            discipline: None,
            category: None,
            table_version: Some(table_version),
            number,
            short_name,
            description,
        }
    }

    pub fn new_grib2(
        discipline: u8,
        category: u8,
        number: u8,
        short_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            discipline: Some(discipline),
            category: Some(category),
            table_version: None,
            number,
            short_name,
            description,
        }
    }
}
