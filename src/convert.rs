//! A collection of useful conversion functions
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
/// If no suffix is provided it will assume inches. (Sorry this pains me too, but quilters mostly seem to be americans,
/// and americans use inches for everything.)
///
/// ```
/// use esvg::convert::parse_length;
/// assert_eq!(parse_length("27in", 96).unwrap(), 2592);
/// assert_eq!(parse_length("2.5mm", 96).unwrap(), 9);
/// assert_eq!(parse_length("2 4/16in", 96).unwrap(), 216);
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
            // Inches are often provided in fractions so lets try and figure those out.
            // examples: "1 1/2in", "5/8in", "3/16in", "7 1/4in", "1.544in", "1in"
            parse_inches(value, dpi)
        }
        "px" => {
            let px = i32::from_str(&value[..l - 2])?;
            Ok(px)
        }
        _ => parse_inches(value, dpi),
    }
}

fn parse_inches(value: &str, dpi: i32) -> Result<i32, error::Error> {
    let l = value.len();

    let numeric_part: &str = if value.ends_with("in") {
        &value[..l - 2]
    } else {
        value
    };

    let inches = if numeric_part.contains(' ') || numeric_part.contains('/') {
        let (whole_inches, remainder) = if let Some(i) = numeric_part.find(' ') {
            // some kind of whole number
            (f64::from_str(numeric_part[..i].trim())?, &numeric_part[i..])
        } else {
            (0.0, numeric_part)
        };

        // fractional part
        let (top, bottom) = if let Some(i) = remainder.find('/') {
            (
                f64::from_str(remainder[..i].trim())?,
                f64::from_str(remainder[i + 1..].trim())?,
            )
        } else {
            (1.0, 1.0)
        };

        whole_inches + (top / bottom)
    } else {
        f64::from_str(numeric_part)?
    };
    Ok(inches_to_pixels(inches, dpi))
}

/// Convert a pixel length into a specified unit.
/// Supports "mm", "cm", "in", and "px" values for units
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

/// get the unit suffix from a string, if it has one.
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

/// Parse an angle in degrees into radians
pub fn parse_angle(value: &str) -> Result<f64, error::Error> {
    let angle = f64::from_str(value)?;
    if !(0.0..=360.0).contains(&angle) {
        Err(error::Error::AngleOutOfRange(angle))
    } else {
        Ok(angle.to_radians())
    }
}

/// Parse a hex string style colour into an R, G, B, A tuple between 0 and 1
/// If no alpha channel is provided then this will assume 1.0
/// Note: Does not support three character hex codes
///
/// ```
/// let (r, g, b, a) = esvg::convert::parse_colour("#FF00AA33").unwrap();
/// assert_eq!(r, 1.0);
/// assert_eq!(g, 0.0);
/// assert_eq!(b, 0.6666666666666666);
/// assert_eq!(a, 0.2);
/// ```
pub fn parse_colour(value: &str) -> Result<(f64, f64, f64, f64), error::Error> {
    if value.len() < 6 {
        return Err(error::Error::ColourError(value.to_string()));
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

/// 30 degrees as radians
pub const DEG_30: f64 = 30.0 * (PI / 180.0);
/// 45 degrees as radians
pub const DEG_45: f64 = 45.0 * (PI / 180.0);
/// 60 degrees as radians
pub const DEG_60: f64 = 60.0 * (PI / 180.0);
/// 90 degrees as radians
pub const DEG_90: f64 = 90.0 * (PI / 180.0);
/// 120 degrees as radians
pub const DEG_120: f64 = 120.0 * (PI / 180.0);
/// 180 degrees as radians
pub const DEG_180: f64 = 180.0 * (PI / 180.0);
/// 270 degrees as radians
pub const DEG_270: f64 = 270.0 * (PI / 180.0);
/// 360 degrees as radians
pub const DEG_360: f64 = 360.0 * (PI / 180.0);

#[cfg(test)]
mod tests {

    use crate::convert::parse_length;

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

    #[test]
    pub fn parse_length_valid() {
        let value = parse_length("2.5", 96).unwrap();
        assert_eq!(value, 240);

        let value = parse_length("2.5cm", 96).unwrap();
        assert_eq!(value, 94);

        let value = parse_length("2.5mm", 96).unwrap();
        assert_eq!(value, 9);

        let value = parse_length("2 4/8in", 96).unwrap();
        assert_eq!(value, 240);

        let value = parse_length("1", 96).unwrap();
        assert_eq!(value, 96);

        let value = parse_length("5/10", 96).unwrap();
        assert_eq!(value, 48);
    }
}
