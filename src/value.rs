use std::rc::Rc;
use data::{Value, List, Scope, RuntimeError};

impl Value
{
    pub fn type_name(&self) -> &'static str
    {
        match *self {
            Value::Nil => "Nil",
            Value::Number(_) => "Number",
            Value::Symbol(_) => "Symbol",
            Value::String(_) => "String",
            Value::Function(_) => "Function",
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

    pub fn eval(&self, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Symbol(ref name) => env.get(name).ok_or(RuntimeError::UnkSymbol(name.clone())),
            Value::List(ref lst) => lst.call(env),
            _ => Ok(self.clone()),
        }
    }

    pub fn call(&self, args: &List, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Function(ref func) => func.call(args, env),
            ref other => Err(RuntimeError::InvalidCall(other.type_name())),
        }
    }
}
