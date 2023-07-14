//! Helper functions for creating particular shapes
use crate::Element;
use ::polygonical::point::Point;

// create a circle element
pub fn circle(p: Point, radius: i32) -> Element {
    let mut el = Element::new("circle");
    el.set("cx", p.x);
    el.set("cy", p.y);
    el.set("r", radius);
    el.set("fill", "none");
    el
}

/// Create a series of circle elements at each of the points provided with the given radius
/// The provided circles will be wrapped in their own group element
pub fn many_circles(points: Vec<Point>, radius: i32) -> Element {
    let mut group = Element::new("g");

    for p in points {
        group.add(&circle(p, radius));
    }

    group
}

/// Create a rectangle element centred on point with the given width and height
pub fn rectangle(center: Point, width: f64, height: f64) -> Element {
    let mut el = Element::new("rect");
    el.set("x", center.x - (width / 2.0));
    el.set("y", center.y - (height / 2.0));
    el.set("width", width);
    el.set("height", height);
    el
}

/// Create a rectangle with rounded corners
pub fn rounded_rectangle(center: Point, width: f64, height: f64, rounding: f64) -> Element {
    let mut el = rectangle(center, width, height);
    el.set("rx", rounding);
    el.set("ry", rounding);

    el
}

/// Create an ellipse.
/// Note: with svg's there is not a way to create an off axis ellipse.
/// You will need to use the transform functions
pub fn ellipse(center: Point, rx: f64, ry: f64) -> Element {
    let mut el = Element::new("ellipse");
    el.set("cx", center.x);
    el.set("cy", center.y);
    el.set("rx", rx);
    el.set("ry", ry);
    el
}
