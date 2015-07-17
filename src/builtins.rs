use data::{Value, List};
use data::RuntimeError::*;
use scope::GlobalScope;

macro_rules! try_cast
{
    ($e:expr, $ty:ident) => (match $e {
        Value::$ty(val) => val,
        _ => return Err(InvalidArgType(stringify!($ty), $e.type_name()))
    })
}

pub fn load(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args, _| {
        args.iter().next().ok_or(InvalidArgNum(1, 0))
    });

    env.set_builtin("set", false, |args, env| {
        let mut iter = args.iter();
        let key = try!(iter.next().ok_or(InvalidArgNum(2, 0)));
        let val = try!(iter.next().ok_or(InvalidArgNum(2, 1)));
        let ev = try!(val.eval(env));
        env.set(&*try_cast!(key, Symbol), ev.clone());
        Ok(ev)
    });

    env.set_builtin("funcall", true, |args, env| args.call(env));

    env.set_builtin("apply", true, |args, env| {
        let mut iter = args.iter();
        let func = try!(iter.next().ok_or(InvalidArgNum(2, 0)));
        let lst = try!(iter.next().ok_or(InvalidArgNum(2, 1)));
        try!(func.eval(env)).call(&try_cast!(lst, List), env)
    });

    env.set_builtin("map", true, |args, env| {
        let mut iter = args.iter();
        let first = try!(iter.next().ok_or(InvalidArgNum(2, 0)));
        let lst = try!(iter.next().ok_or(InvalidArgNum(2, 1)));
        let func = try_cast!(first, Builtin); //FIXME: make a Function trait
        try_cast!(lst, List).iter().map(|val| func.call(&val.wrap(), env)).collect::<Result<_, _>>()
            .map(|vec| Value::List(List::from_vec(vec)))
    });

    env.set_builtin("fold", true, |args, env| {
        let mut iter = args.iter();
        let first = try!(iter.next().ok_or(InvalidArgNum(3, 0)));
        let init = try!(iter.next().ok_or(InvalidArgNum(3, 1)));
        let lst = try!(iter.next().ok_or(InvalidArgNum(3, 2)));
        let func = try_cast!(first, Builtin); // ^ same
        try_cast!(lst, List).fold(init, |acc, val| func.call(&List::cons(acc, val.wrap()), env))
    });

    env.set_builtin("+", true, |args, _| {
        args.fold(0.0, |acc, val| Ok(acc + try_cast!(val, Number))).map(|n| Value::Number(n))
    });

    env.set_builtin("*", true, |args, _| {
        args.fold(1.0, |acc, val| Ok(acc * try_cast!(val, Number))).map(|n| Value::Number(n))
    });
}
