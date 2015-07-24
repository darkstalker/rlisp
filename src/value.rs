use std::rc::Rc;
use std::cell::RefCell;
use data::{Value, List, Scope, RuntimeError};

impl Value
{
    pub fn type_name(&self) -> &'static str
    {
        match *self {
            Value::Nil => "Nil",
            Value::Bool(_) => "Bool",
            Value::Number(_) => "Number",
            Value::Symbol(_) => "Symbol",
            Value::String(_) => "String",
            Value::Builtin(_) | Value::Lambda(_) => "Function",
            Value::List(_) => "List",
        }
    }

    pub fn quote(self) -> Value
    {
        Value::List(List::cons(Value::Symbol(Rc::new("quote".to_string())), self.wrap()))
    }

    pub fn wrap(self) -> List
    {
        List::cons(self, List::End)
    }

    pub fn eval(&self, env: Rc<RefCell<Scope>>) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Symbol(ref name) => env.borrow().get(name).ok_or(RuntimeError::UnkSymbol(name.clone())),
            Value::List(ref lst) => lst.call(env),
            _ => Ok(self.clone()),
        }
    }
}
