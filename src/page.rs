//! Things to do with the document being created, its size, its borders, etc
use polygonical::point::Point;

use crate::convert;
use crate::error::Error;

/// Describe borders around the edge of a page. These don't prevent you drawing off the side of the page
/// they are here to help you keep track of them, they can freely be set to zero.
pub struct Borders {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Borders {
    /// Create a default border of half an inch on all sides
    pub fn default(dpi: i32) -> Borders {
        Borders::even(0.5, dpi)
    }

    /// Create a border of size on all four sides
    pub fn even(size: f64, dpi: i32) -> Borders {
        Borders {
            top: convert::inches_to_pixels(size, dpi),
            bottom: convert::inches_to_pixels(size, dpi),
            left: convert::inches_to_pixels(size, dpi),
            right: convert::inches_to_pixels(size, dpi),
        }
    }

    pub fn rotate(&self) -> Borders {
        Borders {
            top: self.left,
            bottom: self.right,
            left: self.bottom,
            right: self.top,
        }
    }
}

macro_rules! paper_size {
    // There isn't a way to concatenate identifiers in function names so we have to provide both.
    // this sucks, but the long_name should always be equal to ${name}_with_border
    ($($name:ident, $long_name:ident: $width:expr, $height:expr,)*) => {
        $(
            #[allow(non_snake_case)]
            pub fn $name(dpi:i32) -> Page {
                Page::$long_name(dpi, Borders::default(dpi))
            }

            #[allow(non_snake_case)]
            pub fn $long_name(dpi: i32, border: Borders) -> Page {
                Page {
                    dpi,
                    width: convert::inches_to_pixels($width, dpi),
                    height: convert::inches_to_pixels($height, dpi),

                    borders: border,
                }
            }
        )*
    };
}

/// Page is used to define the size of an svg you wish to create.
pub struct Page {
    /// Pixels per inch for calculating conversions
    pub dpi: i32,
    /// Width of the page in pixels
    pub width: i32,
    /// Height of the page in pixels
    pub height: i32,
    /// Borders represent the padding around the edge of the page that should be left alone.
    /// Note: it is up to you to deal with this, nothing will stop you drawing over the border areas
    pub borders: Borders,
}

impl Page {
    /// Construct a page given a name for it and the dpi and margin information.
    ///
    /// The name can either be something like A3 or Letter or 200mmx200in to create a 200 millimetre by 200 inch svg
    /// Margin is always in inches. It doesn't stop you drawing over the edge of the page.
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
                Err(Error::UnknownPaper(name.to_string()))
            }
        } else {
            match name.to_lowercase().as_str() {
                // TODO: add more paper sizes
                "a5" => Ok(Page::A5_with_border(dpi, border)),
                "a4" => Ok(Page::A4_with_border(dpi, border)),
                "a3" => Ok(Page::A3_with_border(dpi, border)),
                "letter" => Ok(Page::letter_with_border(dpi, border)),
                _ => Err(Error::UnknownPaper(name.to_string())),
            }
        }
    }

    // Create paper sizes using macros to avoid duplication
    // TODO: add more paper sizes (make sure to keep with the same pattern)
    paper_size!(
        A5, A5_with_border: 5.8, 8.27,
        A4, A4_with_border: 8.27, 11.7,
        A3, A3_with_border: 11.7, 16.5,
        letter, letter_with_border: 8.5, 11.0,
    );

    /// Rotate this page through 90 degrees, portrait to landscape and landscape to portrait.
    pub fn rotate(&self) -> Page {
        Page {
            dpi: self.dpi,
            width: self.height,
            height: self.width,
            borders: self.borders.rotate(),
        }
    }

    /// returns true if the height of the page is greater than its width
    /// Note: a square page will return false
    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    /// returns true if the width of the page is greater than the height
    /// Note: a square page will return false
    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    /// Return the point in the document that matches to the borders in the top left corner
    pub fn top_left(&self) -> Point {
        Point::new(self.borders.left, self.borders.top)
    }

    /// Return the point in the document that matches to the borders in the top left corner
    pub fn top_right(&self) -> Point {
        Point::new(self.width - self.borders.right, self.borders.top)
    }

    /// Return the point in the document that matches to the borders in the bottom left corner
    pub fn bottom_left(&self) -> Point {
        Point::new(self.borders.left, self.height - self.borders.bottom)
    }

    /// Return the point in the document that matches to the borders in the bottom right corner
    pub fn bottom_right(&self) -> Point {
        Point::new(
            self.width - self.borders.right,
            self.height - self.borders.bottom,
        )
    }

    /// Return the center point of the document including the borders
    pub fn center(&self) -> Point {
        Point::new(
            self.borders.left + ((self.width - self.borders.right - self.borders.left) / 2),
            self.borders.top + ((self.height - self.borders.top - self.borders.bottom) / 2),
        )
    }

    /// Return the point on the left border in the middle vertically
    pub fn center_left(&self) -> Point {
        Point::new(
            self.borders.left,
            self.borders.top + ((self.height - self.borders.top - self.borders.bottom) / 2),
        )
    }

    /// Return the point on the right border in the middle vertically
    pub fn center_right(&self) -> Point {
        Point::new(
            self.width - self.borders.right,
            self.borders.top + ((self.height - self.borders.top - self.borders.bottom) / 2),
        )
    }

    /// Return the point on the top border in the middle horizontally
    pub fn center_top(&self) -> Point {
        Point::new(
            self.borders.left + ((self.width - self.borders.right - self.borders.left) / 2),
            self.borders.top,
        )
    }

    /// Return the point on the bottom border in the middle horizontally
    pub fn center_bottom(&self) -> Point {
        Point::new(
            self.borders.left + ((self.width - self.borders.right - self.borders.left) / 2),
            self.height - self.borders.bottom,
        )
    }

    /// Return the width of the page minus the borders in pixels
    pub fn display_width_px(&self) -> i32 {
        self.width - self.borders.right - self.borders.left
    }

    /// Return the width of the page minus the borders in pixels
    pub fn display_height_px(&self) -> i32 {
        self.height - self.borders.top - self.borders.bottom
    }
}
