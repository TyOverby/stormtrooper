extern crate ares;

mod script;
mod svg;

pub struct Drawing {
    figures: Vec<Figure>,
    default: Box<Fn(f64) -> Unit>
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            figures: vec![],
            default: Box::new(Unit::In) as Box<Fn(f64) -> Unit>
        }
    }

    pub fn push(&mut self, figure: Figure) {
        self.figures.push(figure);
    }

    pub fn default_unit(&self, v: f64) -> Unit {
        (self.default)(v)
    }

    pub fn figures(&self) -> &[Figure] {
        &self.figures
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Figure {
    CutLine(Line),
    DrawLine(Line, Unit)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit {
    Px(f64),
    In(f64),
    Cm(f64),
    Mm(f64),
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("got {:?}", self);
        match *self {
            Unit::Px(x) => write!(f, "{}px", x),
            Unit::In(x) => write!(f, "{}in", x),
            Unit::Cm(x) => write!(f, "{}cm", x),
            Unit::Mm(x) => write!(f, "{}mm", x),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    p1: (Unit, Unit),
    p2: (Unit, Unit)
}

fn main() {
    let mut drawing = Drawing::new();
    script::run_script(&mut drawing, "(cut-line 1 2 3 (mm 4))");
    svg::write_svg(&drawing, &mut std::io::stdout());
}
