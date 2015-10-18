extern crate ares;

mod script;
mod svg;

pub type Drawing = Vec<Figure>;

#[derive(Debug, PartialEq)]
pub enum Figure {
    CutLine(Line),
    DrawLine(Line, f64)
}

#[derive(Debug, PartialEq)]
pub struct Line {
    p1: (f64, f64),
    p2: (f64, f64)
}

fn main() {
    let state = script::run_script("(cut-line 1 2 3 4)");
    svg::write_svg(state, &mut std::io::stdout());
}
