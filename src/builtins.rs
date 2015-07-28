use std::fmt;
use std::rc::Rc;
use std::collections::VecDeque;
use data::{Value, List, Function, RuntimeError};
use data::RuntimeError::*;
use scope::{Scope, RcScope};
use lambda::Lambda;
use list::fold_result;

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub func: Box<Fn(VecDeque<Value>, RcScope) -> Result<Value, RuntimeError>>,
}

impl Function for BuiltinFn
{
    fn call(&self, args: &List, env: RcScope, do_ev: bool) -> Result<Value, RuntimeError>
    {
        if do_ev && self.do_eval
        {
            let ev_args = try!(args.eval(env.clone()));
            (self.func)(ev_args, env)
        }
        else
        {
            (self.func)(args.iter().collect(), env)
        }
    }
}

impl PartialEq for BuiltinFn
{
    fn eq(&self, other: &Self) -> bool
    {
        self as *const _ == other as *const _
    }
}

impl fmt::Debug for BuiltinFn
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

macro_rules! check_arg
{
    ($dq:expr, $num:expr, $cur:expr) => (match $dq.pop_front() {
        Some(val) => val,
        None => return Err(InvalidArgNum($num, $cur)),
    });
    ($dq:expr, $ty:ident, $num:expr, $cur:expr) => (match $dq.pop_front() {
        Some(Value::$ty(val)) => val,
        Some(other) => return Err(InvalidArgType(stringify!($ty), other.type_name())),
        None => return Err(InvalidArgNum($num, $cur)),
    });
}

macro_rules! check_function
{
    ($dq:expr, $num:expr, $cur:expr) => (match $dq.pop_front() {
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

pub fn load_builtins(env: &mut Scope)
{
    env.set_builtin("quote", false, |mut args, _| {
        args.pop_front().ok_or(InvalidArgNum(1, 0))
    });

    #[inline(always)]
    fn assign_impl<F>(mut args: VecDeque<Value>, env: RcScope, f: F) -> Result<Value, RuntimeError>
        where F: Fn(RcScope, &str, Value)
    {
        let key = check_arg!(args, Symbol, 2, 0);
        let val = try!(check_arg!(args, 2, 1).eval(env.clone()));
        f(env, &key, val.clone());
        Ok(val)
    };

    env.set_builtin("let", false, |args, env| assign_impl(args, env, |e, k, v| e.borrow_mut().decl(k, v)));
    env.set_builtin("set", false, |args, env| assign_impl(args, env, |e, k, v| e.borrow_mut().set(k, v)));

    env.set_builtin("funcall", true, |args, env| {
        List::from_de_iter(args.into_iter()).call(env)
    });

    env.set_builtin("apply", true, |mut args, env| {
        let func = check_arg!(args, 2, 0);
        let lst = check_arg!(args, List, 2, 1);
        List::cons(func, lst).call(env)
    });

    env.set_builtin("map", true, |mut args, env| {
        let func = check_function!(args, 2, 0);
        let lst = check_arg!(args, List, 2, 1);
        lst.iter().map(|val| func.call(&val.wrap(), env.clone(), false)).collect::<Result<_, _>>()
            .map(|lst| Value::List(lst))
    });

    env.set_builtin("fold", true, |mut args, env| {
        let func = check_function!(args, 3, 0);
        let init = check_arg!(args, 3, 1);
        let lst = check_arg!(args, List, 3, 2);
        fold_result(lst.iter(), init, |acc, val| func.call(&List::cons(acc, val.wrap()), env.clone(), false))
    });

    env.set_builtin("car", true, |mut args, _| {
        let lst = check_arg!(args, List, 1, 0);
        Ok(match lst {
            List::Node(cons) => cons.car.clone(),
            _ => Value::Nil,
        })
    });

    env.set_builtin("cdr", true, |mut args, _| {
        let lst = check_arg!(args, List, 1, 0);
        Ok(match lst {
            List::Node(cons) => Value::List(cons.cdr.clone()),
            _ => Value::Nil,
        })
    });

    env.set_builtin("cons", true, |mut args, _| {
        let car = check_arg!(args, 2, 0);
        let cdr = check_arg!(args, List, 2, 1);
        Ok(Value::List(List::cons(car, cdr)))
    });

    env.set_builtin("list", true, |args, _| {
        Ok(Value::List(List::from_de_iter(args.into_iter())))
    });

    env.set_builtin("not", true, |mut args, _| {
        let val = check_arg!(args, 1, 0);
        Ok(Value::Bool(match val {
            Value::Nil | Value::Bool(false) => true,
            _ => false,
        }))
    });

    env.set_builtin("and", false, |args, env| {
        let mut last = Value::Bool(true);
        for val in args
        {
            match try!(val.eval(env.clone())) {
                v @ Value::Nil | v @ Value::Bool(false) => return Ok(v),
                other => last = other,
            }
        }
        Ok(last)
    });

    env.set_builtin("or", false, |args, env| {
        let mut last = Value::Bool(false);
        for val in args
        {
            match try!(val.eval(env.clone())) {
                v @ Value::Nil | v @ Value::Bool(false) => last = v,
                other => return Ok(other),
            }
        }
        Ok(last)
    });

    env.set_builtin("if", false, |mut args, env| {
        let cond = check_arg!(args, 2, 0);
        let then = check_arg!(args, 2, 1);
        match try!(cond.eval(env.clone())) {
            Value::Nil | Value::Bool(false) => args.pop_front().unwrap_or(Value::Nil).eval(env),
            _ => then.eval(env),
        }
    });

    env.set_builtin("begin", false, |args, env| {
        let local = Scope::local(env).wrap();
        let mut last = Value::Nil;
        for val in args
        {
            last = try!(val.eval(local.clone()));
        }
        Ok(last)
    });

    env.set_builtin("eval", true, |mut args, env| {
        let expr = check_arg!(args, 1, 0);
        expr.eval(env)
    });

    env.set_builtin("lambda", false, |mut args, env| {
        let arg_lst = check_arg!(args, List, 1, 0);
        arg_lst.iter().map(|val| map_value!(val, Symbol, |n: Rc<String>| (*n).clone()))
            .collect::<Result<_, _>>()
            .map(|names| Value::Lambda(Rc::new(Lambda::new(names, args, env))))
    });

    env.set_builtin("+", true, |args, _| {
        fold_result(args.into_iter(), 0.0, |acc, val| map_value!(val, Number, |n| acc + n)).map(|n| Value::Number(n))
    });

    env.set_builtin("*", true, |args, _| {
        fold_result(args.into_iter(), 1.0, |acc, val| map_value!(val, Number, |n| acc * n)).map(|n| Value::Number(n))
    });

    #[inline(always)]
    fn numeric_op<F, G>(mut args: VecDeque<Value>, ident: f64, op: F) -> Result<Value, RuntimeError>
        where F: Fn(f64) -> G, G: Fn(f64) -> f64
    {
        let num = check_arg!(args, Number, 1, 0);
        if args.is_empty()
        {
            Ok(Value::Number(op(ident)(num)))
        }
        else
        {
            fold_result(args.into_iter(), num, |acc, val| map_value!(val, Number, op(acc)))
                .map(|n| Value::Number(n))
        }
    }

    env.set_builtin("-", true, |args, _| numeric_op(args, 0.0, |lhs| move |rhs| lhs - rhs));
    env.set_builtin("/", true, |args, _| numeric_op(args, 1.0, |lhs| move |rhs| lhs / rhs));

    env.set_builtin("equal", true, |mut args, _| {
        let va = check_arg!(args, 2, 0);
        let vb = check_arg!(args, 2, 1);
        Ok(Value::Bool(va == vb))
    });

    #[inline(always)]
    fn comp_op<F, G>(mut args: VecDeque<Value>, op_num: F, op_str: G) -> Result<Value, RuntimeError>
        where F: Fn(f64, f64) -> bool, G: Fn(&str, &str) -> bool
    {
        let va = check_arg!(args, 2, 0);
        let vb = check_arg!(args, 2, 1);
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

    env.set_builtin("atom", true, |mut args, _| {
        let val = check_arg!(args, 1, 0);
        Ok(Value::Bool(match val {
            Value::List(List::Node(_)) => false,
            _ => true,
        }))
    });

    env.set_builtin("typeof", true, |mut args, _| {
        let val = check_arg!(args, 1, 0);
        Ok(Value::String(Rc::new(val.type_name().to_string())))
    });

    env.set_builtin("display", true, |mut args, _| {
        let val = check_arg!(args, 1, 0);
        println!("{}", val);
        Ok(val)
    });

    env.set_builtin("debug", true, |mut args, _| {
        let val = check_arg!(args, 1, 0);
        println!("{:?}", val);
        Ok(val)
    });
}
