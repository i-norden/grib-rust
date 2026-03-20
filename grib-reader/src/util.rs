pub fn grib_i8(byte: u8) -> i16 {
    let magnitude = (byte & 0x7f) as i16;
    if byte & 0x80 == 0 {
        magnitude
    } else {
        -magnitude
    }
}

pub fn grib_i24(bytes: &[u8]) -> Option<i32> {
    let raw = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    let magnitude = (raw & 0x7f_ffff) as i32;
    Some(if raw & 0x80_0000 == 0 {
        magnitude
    } else {
        -magnitude
    })
}

pub fn grib_i16(bytes: &[u8]) -> Option<i16> {
    let raw = u16::from_be_bytes(bytes.try_into().ok()?);
    let magnitude = (raw & 0x7fff) as i16;
    Some(if raw & 0x8000 == 0 {
        magnitude
    } else {
        -magnitude
    })
}

pub fn grib_i32(bytes: &[u8]) -> Option<i32> {
    let raw = u32::from_be_bytes(bytes.try_into().ok()?);
    let magnitude = (raw & 0x7fff_ffff) as i32;
    Some(if raw & 0x8000_0000 == 0 {
        magnitude
    } else {
        -magnitude
    })
}

#[cfg(test)]
mod tests {
    use super::{grib_i16, grib_i24, grib_i32, grib_i8};

    #[test]
    fn parses_signed_grib_i8() {
        assert_eq!(grib_i8(0x05), 5);
        assert_eq!(grib_i8(0x85), -5);
    }

    #[test]
    fn parses_signed_grib_i16() {
        assert_eq!(grib_i16(&0x0005u16.to_be_bytes()), Some(5));
        assert_eq!(grib_i16(&0x8005u16.to_be_bytes()), Some(-5));
    }

    #[test]
    fn parses_signed_grib_i24() {
        assert_eq!(grib_i24(&[0x00, 0x00, 0x05]), Some(5));
        assert_eq!(grib_i24(&[0x80, 0x00, 0x05]), Some(-5));
    }

    #[test]
    fn parses_signed_grib_i32() {
        assert_eq!(grib_i32(&0x0000_0005u32.to_be_bytes()), Some(5));
        assert_eq!(grib_i32(&0x8000_0005u32.to_be_bytes()), Some(-5));
    }
}
