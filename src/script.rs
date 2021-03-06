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

use super::{Drawing, Figure, Unit};

fn into_number(value: &Value, context: &mut LoadedContext<Drawing>) -> AresResult<Unit> {
    let user_data = match value {
        a@&Value::Int(_) => context.call_named("default-unit", &[a.clone()]),
        a@&Value::Float(_) => context.call_named("default-unit", &[a.clone()]),
        a@&Value::UserData(_) => Ok(a.clone()),
        other => {
            Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "Int or Float or UserData-Unit".into()
            })
        }
    };

    let user_data = try!(user_data);
    if let Value::UserData(ud) = user_data {
        if let Some(unit) = ud.downcast_ref() {
            let unit: &Unit = unit;
            Ok(unit.clone())
        } else {
            Err(AresError::UnexpectedType {
                value: Value::UserData(ud.clone()),
                expected: "Int or Float or UserData-Unit".into()
            })
        }
    } else {
        Err(AresError::UnexpectedType {
            value: user_data.clone(),
            expected: "Int or Float or UserData-Unit".into()
        })
    }
}

fn cut_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 4, "exactly 4"));
    let cut_width = ctx.get("cut-width").unwrap();
    let cut_width = try!(into_number(&cut_width, ctx));
    let p1x = try!(into_number(&args[0], ctx));
    let p1y = try!(into_number(&args[1], ctx));
    let p2x = try!(into_number(&args[2], ctx));
    let p2y = try!(into_number(&args[3], ctx));

    let drawing = ctx.state();
    drawing.push(Figure::Line {
        p1: (p1x, p1y),
        p2: (p2x, p2y),
        width: cut_width,
    });
    Ok(0.into())
}

fn draw_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 5, "exactly 5"));
    let p1x = try!(into_number(&args[0], ctx));
    let p1y = try!(into_number(&args[1], ctx));
    let p2x = try!(into_number(&args[2], ctx));
    let p2y = try!(into_number(&args[3], ctx));
    let width = try!(into_number(&args[4], ctx));

    let drawing = ctx.state();
    drawing.push(Figure::Line {
        p1: (p1x, p1y),
        p2: (p2x, p2y),
        width: width,
    });
    Ok(0.into())
}

fn cut_circle(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 3, "exactly 3"));
    let x = try!(into_number(&args[0], ctx));
    let y = try!(into_number(&args[1], ctx));
    let r = try!(into_number(&args[2], ctx));

    let cut_width = ctx.get("cut-width").unwrap();
    let cut_width = try!(into_number(&cut_width, ctx));

    let drawing = ctx.state();
    drawing.push(Figure::Circle {
        center: (x, y),
        radius: r,
        fill: None,
        width: cut_width
    });

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

pub fn run_script(drawing: &mut Drawing, script: &str) -> AresResult<()> {
    let mut context = Context::new();
    prep_context(&mut context);
    let mut loaded_context = context.load(drawing);
    loaded_context.eval_str(script).map(|_| ())
}

fn prep_context(context: &mut Context<Drawing>) {
    context.set_fn("in", unit_cnv("in", Unit::In));
    context.set_fn("px", unit_cnv("px", Unit::Px));
    context.set_fn("cm", unit_cnv("cm", Unit::Cm));
    context.set_fn("mm", unit_cnv("mm", Unit::Mm));

    context.set_fn("cut-line", user_fn("cut-line", cut_line));
    context.set_fn("draw-line", user_fn("draw-line", draw_line));
    context.set_fn("cut-circle", user_fn("cut-circle", cut_circle));

    let inches = context.get("in");
    context.set("default-unit", inches.unwrap());
    context.set("cut-width", Value::user_data(Unit::Px(0.1)));
}
