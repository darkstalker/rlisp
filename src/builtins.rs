use data::{Value, List, Scope, Function, RuntimeError};
use data::RuntimeError::*;
use scope::{GlobalScope, LocalScope};

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub func: Box<Fn(&List, &mut Scope) -> Result<Value, RuntimeError>>,
}

impl Function for BuiltinFn
{
    fn call(&self, args: &List, env: &mut Scope, do_ev: bool) -> Result<Value, RuntimeError>
    {
        if do_ev && self.do_eval
        {
            (self.func)(&try!(args.eval(env)), env)
        }
        else
        {
            (self.func)(args, env)
        }
    }

    fn get_name(&self) -> &str
    {
        self.name
    }
}

macro_rules! check_arg
{
    ($it:expr, $num:expr, $cur:expr) => (match $it.next() {
        Some(val) => val,
        None => return Err(InvalidArgNum($num, $cur)),
    });
    ($it:expr, $ty:ident, $num:expr, $cur:expr) => (match $it.next() {
        Some(Value::$ty(val)) => val,
        Some(other) => return Err(InvalidArgType(stringify!($ty), other.type_name())),
        None => return Err(InvalidArgNum($num, $cur)),
    });
}

macro_rules! map_value
{
    ($v:expr, $ty:ident, $f:expr) => (match $v {
        Value::$ty(val) => Ok($f(val)),
        _ => return Err(InvalidArgType(stringify!($ty), $v.type_name())),
    })
}

pub fn load_builtins(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args, _| {
        args.iter().next().ok_or(InvalidArgNum(1, 0))
    });

    env.set_builtin("let", false, |args, env| {
        let mut iter = args.iter();
        let key = check_arg!(iter, Symbol, 2, 0);
        let val = try!(check_arg!(iter, 2, 1).eval(env));
        env.decl(&key, val.clone());
        Ok(val)
    });

    env.set_builtin("set", false, |args, env| {
        let mut iter = args.iter();
        let key = check_arg!(iter, Symbol, 2, 0);
        let val = try!(check_arg!(iter, 2, 1).eval(env));
        env.set(&key, val.clone());
        Ok(val)
    });

    env.set_builtin("funcall", true, |args, env| args.call(env));

    env.set_builtin("apply", true, |args, env| {
        let mut iter = args.iter();
        let func = check_arg!(iter, 2, 0);
        let lst = check_arg!(iter, List, 2, 1);
        List::cons(func, lst).call(env)
    });

    env.set_builtin("map", true, |args, env| {
        let mut iter = args.iter();
        let func = check_arg!(iter, Function, 2, 0);
        let lst = check_arg!(iter, List, 2, 1);
        lst.iter().map(|val| func.call(&val.wrap(), env, false)).collect::<Result<_, _>>()
            .map(|lst| Value::List(lst))
    });

    env.set_builtin("fold", true, |args, env| {
        let mut iter = args.iter();
        let func = check_arg!(iter, Function, 3, 0);
        let init = check_arg!(iter, 3, 1);
        let lst = check_arg!(iter, List, 3, 2);
        lst.fold(init, |acc, val| func.call(&List::cons(acc, val.wrap()), env, false))
    });

    env.set_builtin("car", true, |args, _| {
        let mut iter = args.iter();
        let lst = check_arg!(iter, List, 1, 0);
        Ok(match lst {
            List::Node(cons) => cons.car.clone(),
            _ => Value::Nil,
        })
    });

    env.set_builtin("cdr", true, |args, _| {
        let mut iter = args.iter();
        let lst = check_arg!(iter, List, 1, 0);
        Ok(match lst {
            List::Node(cons) => Value::List(cons.cdr.clone()),
            _ => Value::Nil,
        })
    });

    env.set_builtin("cons", true, |args, _| {
        let mut iter = args.iter();
        let car = check_arg!(iter, 2, 0);
        let cdr = check_arg!(iter, List, 2, 1);
        Ok(Value::List(List::cons(car, cdr)))
    });

    env.set_builtin("list", true, |args, _| {
        Ok(Value::List(args.clone()))
    });

    env.set_builtin("not", true, |args, _| {
        let mut iter = args.iter();
        let val = check_arg!(iter, 1, 0);
        Ok(Value::Bool(match val {
            Value::Nil | Value::Bool(false) => true,
            _ => false,
        }))
    });

    env.set_builtin("and", false, |args, env| {
        let mut iter = args.iter();
        let mut last = Value::Bool(true);
        while let Some(val) = iter.next()
        {
            match try!(val.eval(env)) {
                v @ Value::Nil | v @ Value::Bool(false) => return Ok(v),
                other => last = other,
            }
        }
        Ok(last)
    });

    env.set_builtin("or", false, |args, env| {
        let mut iter = args.iter();
        let mut last = Value::Bool(false);
        while let Some(val) = iter.next()
        {
            match try!(val.eval(env)) {
                v @ Value::Nil | v @ Value::Bool(false) => last = v,
                other => return Ok(other),
            }
        }
        Ok(last)
    });

    env.set_builtin("if", false, |args, env| {
        let mut iter = args.iter();
        let cond = check_arg!(iter, 2, 0);
        let then = check_arg!(iter, 2, 1);
        match try!(cond.eval(env)) {
            Value::Nil | Value::Bool(false) => iter.next().unwrap_or(Value::Nil).eval(env),
            _ => then.eval(env),
        }
    });

    env.set_builtin("begin", false, |args, env| {
        let mut local = LocalScope::new(env);
        args.iter().map(|val| val.eval(&mut local)).last().unwrap_or(Ok(Value::Nil))
    });

    env.set_builtin("+", true, |args, _| {
        args.fold(0.0, |acc, val| map_value!(val, Number, |n| acc + n)).map(|n| Value::Number(n))
    });

    env.set_builtin("*", true, |args, _| {
        args.fold(1.0, |acc, val| map_value!(val, Number, |n| acc * n)).map(|n| Value::Number(n))
    });
}
