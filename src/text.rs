use crate::{Element, Node};
use ::polygonical::point::Point;

pub fn create_text(text: String, loc: Point, style: &str) -> Element {
    let mut el = Element::new("text");
    let n = Node::Text(text);

    el.add_node(n);
    el.set("x", loc.x);
    el.set("y", loc.y);
    el.set("style", style);

    el
}

// TODO: colour representation structs
pub fn create_text_style(
    font_family: &str,
    font_size: i32,
    font_weight: &str,
    stroke_width: f64,
    fill: &str,
    stroke: &str,
    stroke_opacity: f64,
) -> String {
    format!(
        "font-family:{font_family};font-size:{font_size};font-weight:{font_weight};stroke-width:{stroke_width};fill:{fill};stroke:{stroke};stroke-opacity:{stroke_opacity};"
    )
}
