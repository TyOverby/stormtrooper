#![allow(dead_code)]

extern crate ares;
extern crate notify;
extern crate hyper;
extern crate websocket;

mod script;
mod svg;
mod viewer;

pub struct Drawing {
    figures: Vec<Figure>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            figures: vec![],
        }
    }

    pub fn push(&mut self, figure: Figure) {
        self.figures.push(figure);
    }

    pub fn figures(&self) -> &[Figure] {
        &self.figures
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Figure {
    Line {
        p1: (Unit, Unit),
        p2: (Unit, Unit),
        width: Unit
    }
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
        match *self {
            Unit::Px(x) => write!(f, "{}px", x),
            Unit::In(x) => write!(f, "{}in", x),
            Unit::Cm(x) => write!(f, "{}cm", x),
            Unit::Mm(x) => write!(f, "{}mm", x),
        }
    }
}

fn main() {
    /*
    let mut drawing = Drawing::new();
    script::run_script(&mut drawing, "(cut-line 1 2 3 (mm 4))");
    svg::write_svg(&drawing, &mut std::io::stdout()).unwrap();
    */

    viewer::start();
}
