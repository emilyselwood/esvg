//! Helper functions for handling text
use std::fmt::Display;

use crate::{error::Error, Element, Node};
use ::polygonical::point::Point;
use font_kit::{handle::Handle, source::SystemSource};
use polygonical::boundingbox::BoundingBox;
use rusttype::{point, Font, Scale};

pub struct TextStyle {
    pub font_family: String,
    pub font_size: i32,
    pub font_weight: String,
    pub stroke_width: f64,
    pub fill: String,
    pub stroke: String,
    pub stroke_opacity: f64,
}

impl TextStyle {
    pub fn new(
        font_family: &str,
        font_size: i32,
        font_weight: &str,
        stroke_width: f64,
        fill: &str,
        stroke: &str,
        stroke_opacity: f64,
    ) -> Self {
        TextStyle {
            font_family: font_family.to_string(),
            font_size,
            font_weight: font_weight.to_string(),
            stroke_width,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            stroke_opacity,
        }
    }
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "font-family:{};font-size:{};font-weight:{};stroke-width:{};fill:{};stroke:{};stroke-opacity:{};",
            self.font_family,
            self.font_size,
            self.font_weight,
            self.stroke_width,
            self.fill,
            self.stroke,
            self.stroke_opacity,
        ).as_str())
    }
}

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
#[deprecated(since = "0.4.0", note = "Please use TextStyle structs instead.")]
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

/**
 * Given some text and style information work out the bounding box that the text will take up.
 *
 * Note: This will need to load the font from disk. There is no cache here. If you need to do
 * this a lot it will likely be slow, if needed please raise an issue for it.
 *
 * Note: This does not handle bold or other modifiers in the style.
 */
pub fn find_text_size(text: &str, style: TextStyle) -> Result<BoundingBox, Error> {
    let font_path = find_font(style.font_family)?;
    let data = std::fs::read(&font_path)?;
    let font = Font::try_from_bytes(&data).ok_or(Error::FontLoadingError)?;

    let scale = Scale::uniform(style.font_size as f32);
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    Ok(BoundingBox::new(
        Point::new(0, 0),
        Point::new(glyphs_width, glyphs_height),
    ))
}

/**
 * Get the file path to a system font.
 */
fn find_font(font_name: String) -> Result<String, Error> {
    let handle = SystemSource::new().select_by_postscript_name(font_name.as_str())?;

    match handle {
        Handle::Path { path, .. } => return Ok(path.to_str().unwrap().to_string()),
        _ => Err(Error::FontMemoryFont(font_name)),
    }
}
