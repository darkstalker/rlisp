use std::rc::Rc;
use data::{Value, Cons, Scope, RuntimeError};

impl Value
{
    pub fn type_name(&self) -> &'static str
    {
        match *self {
            Value::Nil => "Nil",
            Value::Number(_) => "Number",
            Value::Symbol(_) => "Symbol",
            Value::String(_) => "String",
            Value::Builtin(_) => "Builtin",
            Value::List(_) => "List",
        }
    }

    pub fn quote(self) -> Value
    {
        Value::List(Cons::new(Value::Symbol(Rc::new("quote".to_string())), Cons::new(self, None)))
    }

    pub fn eval(&self, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Symbol(ref name) => env.get(name).ok_or(RuntimeError::UnkSymbol(name.clone())),
            Value::List(ref opt) => match *opt {
                Some(ref cons) => {
                    match try!(cons.car.eval(env)) {
                        Value::Builtin(func) => {
                            if func.do_eval
                            {
                                (func.call)(&try!(eval_list(&cons.cdr, env)), env)
                            }
                            else
                            {
                                (func.call)(&cons.cdr, env)
                            }
                        },
                        other => Err(RuntimeError::InvalidCall(other.type_name())),
                    }
                },
                None => Ok(Value::Nil),
            },
            _ => Ok(self.clone()),
        }
    }
}

fn eval_list(mut iter: &Option<Rc<Cons>>, env: &mut Scope) -> Result<Option<Rc<Cons>>, RuntimeError>
{
    let mut res = Vec::new();
    while let Some(ref cons) = *iter
    {
        res.push(try!(cons.car.eval(env)));
        iter = &cons.cdr;
    }
    Ok(if res.is_empty() { None } else { Cons::from_vec(res) })
}
