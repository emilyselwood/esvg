use crate::convert;
use crate::error::Error;

pub struct Borders {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Borders {
    pub fn default(dpi: i32) -> Borders {
        Borders::even(0.5, dpi)
    }

    pub fn even(size: f64, dpi: i32) -> Borders {
        Borders {
            top: convert::inches_to_pixels(size, dpi),
            bottom: convert::inches_to_pixels(size, dpi),
            left: convert::inches_to_pixels(size, dpi),
            right: convert::inches_to_pixels(size, dpi),
        }
    }
}

pub struct Page {
    pub dpi: i32,
    pub width: i32,
    pub height: i32,

    pub borders: Borders,
}

impl Page {
    pub fn build_page(name: &str, dpi: i32, margin: f64) -> Result<Page, Error> {
        let border = Borders::even(margin, dpi);

        // WARN: if we ever get a paper size with an x this will break. At the moment it is fine.
        if name.contains('x') {
            let parts: Vec<&str> = name.split('x').collect();
            if parts.len() == 2 {
                let width = convert::parse_length(parts[0], dpi)?;
                let height = convert::parse_length(parts[1], dpi)?;
                Ok(Page {
                    dpi,
                    width,
                    height,
                    borders: border,
                })
            } else {
                Err(Error::UnknownPaper)
            }
        } else {
            match name.to_lowercase().as_str() {
                "a4" => Ok(Page::A4_with_border(dpi, border)),
                "a3" => Ok(Page::A3_with_border(dpi, border)),
                "letter" => Ok(Page::letter_with_border(dpi, border)),
                _ => Err(Error::UnknownPaper),
            }
        }
    }

    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A4(dpi: i32) -> Page {
        Page::A4_with_border(dpi, Borders::default(dpi))
    }

    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A4_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(8.27, dpi),
            height: convert::inches_to_pixels(11.7, dpi),

            borders: border,
        }
    }

    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A3(dpi: i32) -> Page {
        Page::A3_with_border(dpi, Borders::default(dpi))
    }

    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A3_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(11.7, dpi),
            height: convert::inches_to_pixels(16.5, dpi),

            borders: border,
        }
    }

    pub fn letter(dpi: i32) -> Page {
        Page::letter_with_border(dpi, Borders::default(dpi))
    }

    pub fn letter_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(8.5, dpi),
            height: convert::inches_to_pixels(11.0, dpi),

            borders: border,
        }
    }
}
