use std::f64;
use std::f64::consts::PI;
use std::str::FromStr;

use crate::error;

/// convert value of inches to pixels given the dpi
pub fn inches_to_pixels(value: f64, dpi: i32) -> i32 {
    (value * (dpi as f64)).round() as i32
}

/// convert value of centimeters to pixels given the dpi
pub fn cm_to_pixels(value: f64, dpi: i32) -> i32 {
    inches_to_pixels(cm_to_inches(value), dpi)
}

/// convert value of millimeters to pixels given the dpi
pub fn mm_to_pixels(value: f64, dpi: i32) -> i32 {
    inches_to_pixels(mm_to_inches(value), dpi)
}

/// Convert centimeters to inches
pub fn cm_to_inches(value: f64) -> f64 {
    (value * 10.0) / 25.4
}

/// Convert millimeters to inches
pub fn mm_to_inches(value: f64) -> f64 {
    (value) / 25.4
}

/// Convert inches to millimeters
pub fn inches_to_mm(value: f64) -> f64 {
    value * 25.4
}

/// Convert inches to centimeters
pub fn inches_to_cm(value: f64) -> f64 {
    (value * 25.4) / 10.0
}

/// Convert a number of pixels to millimeters given the dpi
pub fn pixels_to_mm(value: i32, dpi: i32) -> f64 {
    inches_to_mm(value as f64 / dpi as f64)
}

/// Convert a number of pixels to centimeters given the dpi
pub fn pixels_to_cm(value: i32, dpi: i32) -> f64 {
    inches_to_cm(value as f64 / dpi as f64)
}

/// Convert a number of pixels to inches given the dpi
pub fn pixels_to_inches(value: i32, dpi: i32) -> f64 {
    value as f64 / dpi as f64
}

/// Parse a length. Handles unit suffixes
///
/// ```
/// use esvg::convert::parse_length;
/// assert_eq!(parse_length("27in", 96).unwrap(), 2592);
/// ```
pub fn parse_length(value: &str, dpi: i32) -> Result<i32, error::Error> {
    let l = value.len();

    match extract_unit(value)? {
        "mm" => {
            let mm = f64::from_str(&value[..l - 2])?;
            Ok(mm_to_pixels(mm, dpi))
        }
        "cm" => {
            let cm = f64::from_str(&value[..l - 2])?;
            Ok(cm_to_pixels(cm, dpi))
        }
        "in" => {
            let inches = f64::from_str(&value[..l - 2])?;
            Ok(inches_to_pixels(inches, dpi))
        }
        "px" => {
            let px = i32::from_str(&value[..l - 2])?;
            Ok(px)
        }
        _ => {
            let inches = f64::from_str(value)?;
            Ok(inches_to_pixels(inches, dpi))
        }
    }
}

pub fn px_to_length(value: i32, unit: &str, dpi: i32) -> Result<String, error::Error> {
    match unit {
        "mm" => {
            let mm = pixels_to_mm(value, dpi);
            Ok(format!("{mm:.2}mm"))
        }
        "cm" => {
            let cm = pixels_to_cm(value, dpi);
            Ok(format!("{cm:.2}cm"))
        }
        "in" => {
            let inches = pixels_to_inches(value, dpi);
            Ok(format!("{inches:.2}in"))
        }
        "px" => Ok(format!("{value}px")),
        _ => {
            let inches = pixels_to_inches(value, dpi);
            Ok(format!("{inches:.2}in"))
        }
    }
}

pub fn extract_unit(value: &str) -> Result<&str, error::Error> {
    let l = value.len();
    if l > 2 {
        if value[l - 2..].chars().any(char::is_numeric) {
            Ok("")
        } else {
            Ok(&value[l - 2..])
        }
    } else {
        Ok("")
    }
}

pub fn deg_to_rad(value: f64) -> f64 {
    value * (PI / 180.0)
}

pub fn rad_to_deg(value: f64) -> f64 {
    value * (180.0 / PI)
}

pub fn parse_angle(value: &str) -> Result<f64, error::Error> {
    let angle = f64::from_str(value)?;
    if !(0.0..=360.0).contains(&angle) {
        Err(error::Error::AngleOutOfRange)
    } else {
        Ok(deg_to_rad(angle))
    }
}

pub fn parse_colour(value: &str) -> Result<(f64, f64, f64, f64), error::Error> {
    if value.len() < 6 {
        return Err(error::Error::ColourError);
    }
    let mut start = 0;
    if value.starts_with('#') {
        start = 1;
    }

    let red = i32::from_str_radix(&value[start..start + 2], 16)?;
    let green = i32::from_str_radix(&value[start + 2..start + 4], 16)?;
    let blue = i32::from_str_radix(&value[start + 4..start + 6], 16)?;
    let mut alpha = 255;
    if value.len() > start + 6 {
        alpha = i32::from_str_radix(&value[start + 6..], 16)?;
    }

    Ok((
        red as f64 / 255.0,
        green as f64 / 255.0,
        blue as f64 / 255.0,
        alpha as f64 / 255.0,
    ))
}

pub const DEG_30: f64 = 30.0 * (PI / 180.0);
pub const DEG_45: f64 = 45.0 * (PI / 180.0);
pub const DEG_60: f64 = 60.0 * (PI / 180.0);
pub const DEG_90: f64 = 90.0 * (PI / 180.0);
pub const DEG_120: f64 = 120.0 * (PI / 180.0);
pub const DEG_180: f64 = 180.0 * (PI / 180.0);
pub const DEG_270: f64 = 270.0 * (PI / 180.0);
pub const DEG_360: f64 = 360.0 * (PI / 180.0);

#[cfg(test)]
mod tests {

    use super::parse_colour;
    #[test]
    pub fn colour_conversion_invalid() {
        assert!(parse_colour("invalid").is_err());
        assert!(parse_colour("i").is_err());
        assert!(parse_colour("#i").is_err());
    }

    #[test]
    pub fn colour_conversion_valid() {
        let (r, g, b, a) = parse_colour("#0c0c0c").unwrap();
        assert_eq!(r, 0.047058823529411764);
        assert_eq!(g, 0.047058823529411764);
        assert_eq!(b, 0.047058823529411764);
        assert_eq!(a, 1.0);
    }
}
