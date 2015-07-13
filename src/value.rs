use std::rc::Rc;
use data::{Value, Cons, Scope, RuntimeError};

impl Value
{
    pub fn quote(self) -> Value
    {
        Value::List(Cons::new(Value::Ident("quote".to_string()), Cons::new(self, None)))
    }

    pub fn eval(&self, env: &Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Ident(ref name) => env.get(name).ok_or(RuntimeError::UnkIdent(name.clone())),
            Value::List(ref opt) => match *opt {
                Some(ref cons) => {
                    match try!(cons.car.eval(env)) {
                        Value::Builtin(func, do_eval) => {
                            if do_eval
                            {
                                func.0(&try!(eval_list(&cons.cdr, env)))
                            }
                            else
                            {
                                func.0(&cons.cdr)
                            }
                        },
                        _ => Err(RuntimeError::InvalidCall),
                    }
                },
                None => Ok(Value::Nil),
            },
            _ => Ok(self.clone()),
        }
    }
}

fn eval_list(mut iter: &Option<Rc<Cons>>, env: &Scope) -> Result<Option<Rc<Cons>>, RuntimeError>
{
    let mut res = Vec::new();
    while let Some(ref cons) = *iter
    {
        res.push(try!(cons.car.eval(env)));
        iter = &cons.cdr;
    }
    Ok(if res.is_empty() { None } else { Cons::from_vec(res) })
}
