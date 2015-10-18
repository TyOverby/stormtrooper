use ares::{
    Value,
    AresResult,
    AresError,
    LoadedContext,
    free_fn,
    user_fn,
    Context,
    ForeignFunction
};
use ares::stdlib::util::expect_arity;

use super::{Drawing, Line, Figure, Unit};

fn into_number(value: &Value, drawing: &Drawing) -> AresResult<Unit> {
    match value {
        &Value::Int(i) => Ok(drawing.default_unit(i as f64)),
        &Value::Float(f) => Ok(drawing.default_unit(f)),
        &Value::UserData(ref ud) => {
            if let Some(unit) = ud.downcast_ref() {
                let unit: &Unit = unit;
                Ok(unit.clone())
            } else {
                Err(AresError::UnexpectedType {
                    value: Value::UserData(ud.clone()),
                    expected: "Int or Float or UserData-Unit".into()
                })
            }
        }
        other => {
            Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "Int or Float or UserData-Unit".into()
            })
        }
    }
}

fn cut_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 4, "exactly 4"));
    let drawing = ctx.state();
    let p1x = try!(into_number(&args[0], drawing));
    let p1y = try!(into_number(&args[1], drawing));
    let p2x = try!(into_number(&args[2], drawing));
    let p2y = try!(into_number(&args[3], drawing));

    drawing.push(Figure::CutLine(Line{ p1: (p1x, p1y), p2: (p2x, p2y) }));
    Ok(0.into())
}

fn draw_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 5, "exactly 5"));
    let drawing = ctx.state();
    let width = try!(into_number(&args[0], drawing));
    let p1x = try!(into_number(&args[1], drawing));
    let p1y = try!(into_number(&args[2], drawing));
    let p2x = try!(into_number(&args[3], drawing));
    let p2y = try!(into_number(&args[4], drawing));

    drawing.push(Figure::DrawLine(Line{ p1: (p1x, p1y), p2: (p2x, p2y) }, width));
    Ok(0.into())
}

fn unit_cnv<F: 'static + Fn(f64) -> Unit>(name: &str, typ: F) -> ForeignFunction<Drawing> {
    return free_fn(name, move |args| {
        try!(expect_arity(args, |l| l == 1, "exactly 1"));
        match &args[0] {
            &Value::Int(i) => Ok(Value::user_data(typ(i as f64))),
            &Value::Float(f) => Ok(Value::user_data(typ(f))),
            other => {
                Err(AresError::UnexpectedType {
                    value: other.clone(),
                    expected: "Int or Float".into()
                })
            }
        }
    });
}

pub fn run_script(drawing: &mut Drawing, script: &str) {
    let mut context = Context::new();

    context.set_fn("in", unit_cnv("in", Unit::In));
    context.set_fn("px", unit_cnv("px", Unit::Px));
    context.set_fn("cm", unit_cnv("cm", Unit::Cm));
    context.set_fn("mm", unit_cnv("mm", Unit::Mm));
    context.set_fn("cut-line", user_fn("cut-line", cut_line));
    context.set_fn("draw-line", user_fn("draw-line", draw_line));

    {
        let mut loaded_context = context.load(drawing);
        loaded_context.eval_str(script).unwrap();
    }
}
