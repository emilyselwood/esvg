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
