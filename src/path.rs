//! Helpers for handling Path data
use crate::Element;
use ::polygonical::point::Point;
use ::polygonical::polygon::Polygon;

/// Represents the data attribute of a svg path
pub struct Data {
    segments: Vec<String>,
}

/// Create a data attribute from a series of points
pub fn create(points: &Vec<Point>) -> Element {
    Data::from_points(points).to_path()
}

/// Create a closed loop data attribute from a series of points
pub fn create_closed(points: &Vec<Point>) -> Element {
    Data::from_points(points).close().to_path()
}

/// Create a closed loop data attribute from a polygon
pub fn create_polygon(poly: &Polygon) -> Element {
    Data::from_points(&poly.points).close().to_path()
}

impl Data {
    /// a new empty data element
    pub fn new() -> Self {
        Data { segments: vec![] }
    }

    /// Create the element content from a series of points
    /// Note: there must be more than one point for this to do anything.
    pub fn from_points(points: &Vec<Point>) -> Self {
        let mut data = Data::new();
        if points.len() > 1 {
            data.move_to(points[0]);
            for p in points[1..].iter() {
                data.line_to(*p);
            }
        }

        data
    }

    /// Add a Move To step to this path.
    pub fn move_to(&mut self, p: Point) -> &mut Data {
        self.segments.push(format!("M{:.3} {:.3}", p.x, p.y));

        self
    }

    /// Add a line to step to this path
    pub fn line_to(&mut self, p: Point) -> &mut Data {
        self.segments.push(format!("L{:.3} {:.3}", p.x, p.y));

        self
    }

    /// Add an arc to step to this path
    pub fn arc_to(
        &mut self,
        p: Point,
        rx: i32,
        ry: i32,
        rotation: f64,
        large: bool,
        sweep: bool,
    ) -> &mut Data {
        let lv = match large {
            true => 1,
            false => 0,
        };

        let sv = match sweep {
            true => 1,
            false => 0,
        };

        self.segments.push(format!(
            "A{} {} {:.3} {} {} {:.3} {:.3}",
            rx, ry, rotation, lv, sv, p.x, p.y
        ));

        self
    }

    /// Close the loop of this path.
    pub fn close(&mut self) -> &mut Data {
        self.segments.push("z".to_string());
        self
    }

    /// Turn this data attribute into a string.
    pub fn build(&self) -> String {
        self.segments.join(" ")
    }

    /// build a path element using this data attribute
    pub fn to_path(&self) -> Element {
        let mut el = Element::new("path");
        el.set("fill", "none");
        el.set("d", self.build());
        el
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
