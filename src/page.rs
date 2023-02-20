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
}

/// Page is used to define the size of an svg you wish to create.
pub struct Page {
    pub dpi: i32,
    pub width: i32,
    pub height: i32,

    pub borders: Borders,
}

impl Page {
    /// Construct a page given a name for it and the dpi and margin information.
    ///
    /// The name can either be something like A3 or Letter or 200mmx200in to create a 200 millimeter by 200 inch svg
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

    /// Create an A4 page with default borders
    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A4(dpi: i32) -> Page {
        Page::A4_with_border(dpi, Borders::default(dpi))
    }

    /// Create an A4 page with the provided borders
    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A4_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(8.27, dpi),
            height: convert::inches_to_pixels(11.7, dpi),

            borders: border,
        }
    }

    /// Create an A3 page with default boarders
    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A3(dpi: i32) -> Page {
        Page::A3_with_border(dpi, Borders::default(dpi))
    }

    /// Create an A3 page with the provided borders
    #[allow(non_snake_case)] // Paper names are upper case we should match
    pub fn A3_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(11.7, dpi),
            height: convert::inches_to_pixels(16.5, dpi),

            borders: border,
        }
    }

    /// Create a letter sized page with default boarders
    pub fn letter(dpi: i32) -> Page {
        Page::letter_with_border(dpi, Borders::default(dpi))
    }

    /// Create a letter sized page with the provided borders
    pub fn letter_with_border(dpi: i32, border: Borders) -> Page {
        Page {
            dpi,
            width: convert::inches_to_pixels(8.5, dpi),
            height: convert::inches_to_pixels(11.0, dpi),

            borders: border,
        }
    }

    // TODO: add more paper sizes

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

    // Return the center point of the document including the borders
    pub fn center(&self) -> Point {
        Point::new(
            (self.width - self.borders.right - self.borders.left) / 2,
            (self.height - self.borders.top - self.borders.bottom) / 2,
        )
    }

    pub fn center_left(&self) -> Point {
        Point::new(
            self.borders.left,
            (self.height - self.borders.top - self.borders.bottom) / 2,
        )
    }

    pub fn center_right(&self) -> Point {
        Point::new(
            self.width - self.borders.right,
            (self.height - self.borders.top - self.borders.bottom) / 2,
        )
    }

    pub fn center_top(&self) -> Point {
        Point::new(
            (self.width - self.borders.right - self.borders.left) / 2,
            self.borders.top,
        )
    }

    pub fn center_bottom(&self) -> Point {
        Point::new(
            (self.width - self.borders.right - self.borders.left) / 2,
            self.height - self.borders.bottom,
        )
    }
}
