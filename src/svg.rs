use std::io::Write;
use std::io::Result as IoResult;
use super::{Figure, Drawing, Line};

const SVG_HEADER: &'static str = r#"<?xml version="1.0" encoding="ISO-8859-1" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
    "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve"
         viewBox="0 0 200 50"
         zoomAndPan="disable" preserveAspectRatio="none">
"#;

struct SvgWriter {
    defs: Vec<u8>,
    body: Vec<u8>,
    id_gen: u32,
}

pub fn write_svg<W: Write>(drawing: &Drawing, writer: &mut W) -> IoResult<()> {
    let mut svg_writer = SvgWriter {
        defs: Vec::new(),
        body: Vec::new(),
        id_gen: 0
    };

    for figure in drawing.figures() {
        write_figure(figure, &mut svg_writer);
    }

    try!(write!(writer, "{}", SVG_HEADER));
    try!(write!(writer, "{}", "<defs>"));
    try!(writer.write_all(&svg_writer.defs[..]));
    try!(write!(writer, "{}", "</defs>"));
    try!(writer.write_all(&svg_writer.body[..]));
    try!(write!(writer, "{}", "</svg>"));

    Ok(())
}

fn write_figure(figure: &Figure, writer: &mut SvgWriter) {
    (match *figure {
        Figure::CutLine(Line { p1: (x1, y1), p2: (x2, y2) }) =>
            write!(&mut writer.body, r#"<line stroke="black" stroke-width="10px" x1="{}" y1="{}" x2="{}" y2="{}"/>"#, x1, y1, x2, y2),
        Figure::DrawLine(Line { p1: (x1, y1), p2: (x2, y2) }, width) =>
            write!(&mut writer.body, r#"<line x1="{}" y1="{}" x2="{}" y2="{}"/>"#, x1, y1, x2, y2),
    }).unwrap();
}
