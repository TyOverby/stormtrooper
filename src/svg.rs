use std::io::Write;
use std::io::Result as IoResult;
use super::{Figure, Drawing};

const SVG_HEADER: &'static str = r#"<?xml version="1.0" encoding="ISO-8859-1" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
    "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     xml:space="preserve"
     width="100%"
     height="100%"
     viewBox="0 0 200 50"
     zoomAndPan="disable" preserveAspectRatio="none">
"#;

struct SvgWriter {
    defs: Vec<u8>,
    body: Vec<u8>,
    id_gen: u32,
}

impl SvgWriter {
    fn new() -> SvgWriter {
        SvgWriter {
            defs: vec![],
            body: vec![],
            id_gen: 0
        }
    }
}

pub fn write_svg<W: Write>(drawing: &Drawing, writer: &mut W) -> IoResult<()> {
    let mut svg_writer = SvgWriter::new();

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
        Figure::Line { p1: (x1, y1), p2: (x2, y2), width } => {
            write!(
                &mut writer.body,
                r#"<line stroke="black" x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="{}"/>"#,
                x1, y1, x2, y2, width)
        }
        Figure::Circle { center: (x, y), radius, width, fill } => {
            write!(
                &mut writer.body,
                r#"<circle cx="{}" cy="{}" r="{}" stroke-width="{}" stroke="black" fill="{}" />"#,
                x, y, radius, width, fill.map(|a| a.to_string()).unwrap_or_else(|| "none".into()))
        }
    }).unwrap();
}


#[cfg(test)]
mod test {
    use ::{Unit, Figure};

    fn test_figure(fig: &Figure) -> String {
        use std::io::Write;
        use super::SvgWriter;

        let mut writer = SvgWriter::new();
        super::write_figure(fig, &mut writer);
        let mut buf = vec![];
        buf.write_all(&writer.body[..]).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn test_line() {
        let line = Figure::Line {
            p1: (Unit::Px(5.0),  Unit::Cm(10.0)),
            p2: (Unit::Mm(20.0), Unit::In(40.0)),
            width: Unit::Px(50.0),
        };
        assert_eq!(
            &test_figure(&line),
            r#"<line stroke="black" x1="5px" y1="10cm" x2="20mm" y2="40in" stroke-width="50px"/>"#);
    }

    #[test]
    fn test_circle() {
        let circle = Figure::Circle {
            center: (Unit::Px(5.0),  Unit::Cm(10.0)),
            radius: Unit::In(15.0),
            width: Unit::Px(50.0),
            fill: None
        };

        assert_eq!(
            &test_figure(&circle),
            r#"<circle cx="5px" cy="10cm" r="15in" stroke-width="50px" stroke="black" fill="none" />"#)
    }
}
