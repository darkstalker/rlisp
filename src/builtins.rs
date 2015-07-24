use std::fmt;
use std::rc::Rc;
use data::{Value, List, RcScope, Function, RuntimeError};
use data::RuntimeError::*;
use scope::{GlobalScope, LocalScope};
use lambda::Lambda;

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub func: Box<Fn(&List, RcScope) -> Result<Value, RuntimeError>>,
}

impl Function for BuiltinFn
{
    fn call(&self, args: &List, env: RcScope, do_ev: bool) -> Result<Value, RuntimeError>
    {
        if do_ev && self.do_eval
        {
            let ev_args = try!(args.eval(env.clone()));
            (self.func)(&ev_args, env)
        }
        else
        {
            (self.func)(args, env)
        }
    }
}

impl PartialEq for BuiltinFn
{
    fn eq(&self, other: &Self) -> bool
    {
        self.name == other.name
    }
}

impl fmt::Debug for BuiltinFn
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "BuiltinFn({})", self.name)
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

macro_rules! check_function
{
    ($it:expr, $num:expr, $cur:expr) => (match $it.next() {
        Some(Value::Builtin(func)) => func as Rc<Function>,
        Some(Value::Lambda(func)) => func as Rc<Function>,
        Some(other) => return Err(InvalidArgType("Function", other.type_name())),
        None => return Err(InvalidArgNum($num, $cur)),
    })
}

macro_rules! map_value
{
    ($v:expr, $ty:ident, $f:expr) => (match $v {
        Value::$ty(val) => Ok($f(val)),
        _ => Err(InvalidArgType(stringify!($ty), $v.type_name())),
    })
}

pub fn load_builtins(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args, _| {
        args.iter().next().ok_or(InvalidArgNum(1, 0))
    });

    #[inline(always)]
    fn assign_impl<F>(args: &List, env: RcScope, f: F) -> Result<Value, RuntimeError>
        where F: Fn(RcScope, &str, Value)
    {
        let mut iter = args.iter();
        let key = check_arg!(iter, Symbol, 2, 0);
        let val = try!(check_arg!(iter, 2, 1).eval(env.clone()));
        f(env, &key, val.clone());
        Ok(val)
    };

    env.set_builtin("let", false, |args, env| assign_impl(args, env, |e, k, v| e.borrow_mut().decl(k, v)));
    env.set_builtin("set", false, |args, env| assign_impl(args, env, |e, k, v| e.borrow_mut().set(k, v)));

    env.set_builtin("funcall", true, |args, env| args.call(env));

    env.set_builtin("apply", true, |args, env| {
        let mut iter = args.iter();
        let func = check_arg!(iter, 2, 0);
        let lst = check_arg!(iter, List, 2, 1);
        List::cons(func, lst).call(env)
    });

    env.set_builtin("map", true, |args, env| {
        let mut iter = args.iter();
        let func = check_function!(iter, 2, 0);
        let lst = check_arg!(iter, List, 2, 1);
        lst.iter().map(|val| func.call(&val.wrap(), env.clone(), false)).collect::<Result<_, _>>()
            .map(|lst| Value::List(lst))
    });

    env.set_builtin("fold", true, |args, env| {
        let mut iter = args.iter();
        let func = check_function!(iter, 3, 0);
        let init = check_arg!(iter, 3, 1);
        let lst = check_arg!(iter, List, 3, 2);
        lst.fold(init, |acc, val| func.call(&List::cons(acc, val.wrap()), env.clone(), false))
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
            match try!(val.eval(env.clone())) {
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
            match try!(val.eval(env.clone())) {
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
        match try!(cond.eval(env.clone())) {
            Value::Nil | Value::Bool(false) => iter.next().unwrap_or(Value::Nil).eval(env),
            _ => then.eval(env),
        }
    });

    env.set_builtin("begin", false, |args, env| {
        let local = LocalScope::new(env).wrap();
        args.eval_to_value(local)
    });

    env.set_builtin("eval", true, |args, env| {
        let mut iter = args.iter();
        let expr = check_arg!(iter, 1, 0);
        expr.eval(env)
    });

    env.set_builtin("lambda", false, |args, env| {
        let (arg_lst, code) = match *args {
            List::Node(ref cons) => match cons.car {
                Value::List(ref lst) => (lst, cons.cdr.clone()),
                ref other => return Err(InvalidArgType("List", other.type_name())),
            },
            List::End => return Err(InvalidArgNum(1, 0)),
        };
        arg_lst.iter().map(|val| map_value!(val, Symbol, |n: Rc<String>| (*n).clone()))
            .collect::<Result<_, _>>()
            .map(|names| Value::Lambda(Rc::new(Lambda::new(names, code, env))))
    });

    env.set_builtin("+", true, |args, _| {
        args.fold(0.0, |acc, val| map_value!(val, Number, |n| acc + n)).map(|n| Value::Number(n))
    });

    env.set_builtin("*", true, |args, _| {
        args.fold(1.0, |acc, val| map_value!(val, Number, |n| acc * n)).map(|n| Value::Number(n))
    });

    #[inline(always)]
    fn numeric_op<F, G>(args: &List, ident: f64, op: F) -> Result<Value, RuntimeError>
        where F: Fn(f64) -> G, G: Fn(f64) -> f64
    {
        match *args {
            List::Node(ref c1) => match c1.cdr {
                List::Node(_) => match c1.car {
                    Value::Number(num) => c1.cdr.fold(num, |acc, val| map_value!(val, Number, op(acc))),
                    ref other => Err(InvalidArgType("Number", other.type_name())),
                },
                List::End => map_value!(c1.car, Number, op(ident)),
            },
            List::End => Err(InvalidArgNum(1, 0)),
        }.map(|n| Value::Number(n))
    }

    env.set_builtin("-", true, |args, _| numeric_op(args, 0.0, |lhs| move |rhs| lhs - rhs));
    env.set_builtin("/", true, |args, _| numeric_op(args, 1.0, |lhs| move |rhs| lhs / rhs));

    env.set_builtin("equal", true, |args, _| {
        let mut iter = args.iter();
        let va = check_arg!(iter, 2, 0);
        let vb = check_arg!(iter, 2, 1);
        Ok(Value::Bool(va == vb))
    });

    #[inline(always)]
    fn comp_op<F, G>(args: &List, op_num: F, op_str: G) -> Result<Value, RuntimeError>
        where F: Fn(f64, f64) -> bool, G: Fn(&str, &str) -> bool
    {
        let mut iter = args.iter();
        let va = check_arg!(iter, 2, 0);
        let vb = check_arg!(iter, 2, 1);
        match (va, vb) {
            (Value::Number(a), Value::Number(b)) => Ok(op_num(a, b)),
            (Value::String(ref a), Value::String(ref b)) => Ok(op_str(a, b)),
            (a, b) => Err(InvalidComp(a.type_name(), b.type_name())),
        }.map(|b| Value::Bool(b))
    }

    env.set_builtin("<", true, |args, _| comp_op(args, |a, b| a < b, |a, b| a < b));
    env.set_builtin(">", true, |args, _| comp_op(args, |a, b| a > b, |a, b| a > b));
    env.set_builtin("<=", true, |args, _| comp_op(args, |a, b| a <= b, |a, b| a <= b));
    env.set_builtin(">=", true, |args, _| comp_op(args, |a, b| a >= b, |a, b| a >= b));

    env.set_builtin("atom", true, |args, _| {
        let mut iter = args.iter();
        let val = check_arg!(iter, 1, 0);
        Ok(Value::Bool(match val {
            Value::List(List::Node(_)) => false,
            _ => true,
        }))
    });

    env.set_builtin("typeof", true, |args, _| {
        let mut iter = args.iter();
        let val = check_arg!(iter, 1, 0);
        Ok(Value::String(Rc::new(val.type_name().to_string())))
    });
}
