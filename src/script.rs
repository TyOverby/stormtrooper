use ares::{Value, AresResult, AresError, LoadedContext, user_fn, Context};
use ares::stdlib::util::expect_arity;

use super::{Drawing, Line, Figure};

fn into_number(value: &Value) -> AresResult<f64> {
    match value {
        &Value::Int(i) => Ok(i as f64),
        &Value::Float(f) => Ok(f),
        other => {
            Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "Int or Float".into()
            })
        }
    }
}

fn cut_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 4, "exactly 4"));
    let p1x = try!(into_number(&args[0]));
    let p1y = try!(into_number(&args[1]));
    let p2x = try!(into_number(&args[2]));
    let p2y = try!(into_number(&args[3]));

    ctx.state().push(Figure::CutLine(Line{ p1: (p1x, p1y), p2: (p2x, p2y) }));
    Ok(0.into())
}

fn draw_line(args: &[Value], ctx: &mut LoadedContext<Drawing>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 5, "exactly 5"));
    let width = try!(into_number(&args[0]));
    let p1x = try!(into_number(&args[1]));
    let p1y = try!(into_number(&args[2]));
    let p2x = try!(into_number(&args[3]));
    let p2y = try!(into_number(&args[4]));

    ctx.state().push(Figure::DrawLine(Line{ p1: (p1x, p1y), p2: (p2x, p2y) }, width));
    Ok(0.into())
}


pub fn run_script(script: &str) -> Drawing {
    let mut state = vec![];
    let mut context = Context::new();

    context.set_fn("cut-line", user_fn("cut-line", cut_line));
    context.set_fn("draw-line", user_fn("draw-line", draw_line));
    {
        let mut loaded_context = context.load(&mut state);
        loaded_context.eval_str(script).unwrap();
    }
    state
}
