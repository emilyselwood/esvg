pub type Colour = String;

pub fn style_stroke_colour(stroke: Colour) -> String {
    format!("stroke:{stroke};")
}

pub fn style_fill(fill: Colour) -> String {
    format!("fill:{fill};")
}

pub fn style_stroke(stroke: Colour, width: f64, opacity: f64) -> String {
    format!("stroke:{stroke};stroke-width:{width};stroke-opacity:{opacity};")
}

// TODO: something to parse style strings into some kind of struct/map

#[cfg(test)]
mod tests {
    use crate::style::style_stroke;

    #[test]
    fn test_styles() {
        assert_eq!(
            style_stroke("black".to_string(), 1.0, 1.0),
            "stroke:black;stroke-width:1;stroke-opacity:1;"
        );
    }
}
