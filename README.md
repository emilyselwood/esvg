# esvg

A document object model based SVG library for construction of vector graphics.

Access is available to the attributes and tags allowing you to construct any SVG you need.

Uses [Polygonical](https://docs.rs/polygonical/) for its shape representation

## Examples

Construct a document and draw a circle
```rust

use esvg::page::Page;
use esvg::{create_document, Element};
use polygonical::point::Point;

let page = Page::A4(96);  // 96 dpi
let mut doc = create_document(&page);
let mut group = Element::new("g");
let circle = esvg::shapes::circle(Point::new(100.0, 100.0), 50);

group.add(&circle);
doc.add(&group);

let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.0//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">
<svg height=\"297.1270833333333mm\" viewBox=\"0, 0, 794, 1123\" width=\"210.07916666666668mm\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
\t<g>
\t\t<circle cx=\"100\" cy=\"100\" fill=\"none\" r=\"50\" />
\t</g>
</svg>
";

assert_eq!(doc.to_pretty_string(), expected);

```


## Features

* Constructing SVGs in memory
* Reading SVGs (Including comments and text nodes)
* Writing SVGs (Including comments and text nodes)
* Path objects
* Text objects
* Circles
* Common page sizes built in

## Wanted features

* Path data to polygons

## Things we explicitly won't support

* Converting SVGs to other formats. 