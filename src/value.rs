use std::rc::Rc;
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
}

impl PartialEq for Value
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other) {
            (&Value::Nil, &Value::Nil) => true,
            (&Value::Bool(a), &Value::Bool(b)) => a == b,
            (&Value::Number(a), &Value::Number(b)) => a == b,
            (&Value::Symbol(ref a), &Value::Symbol(ref b)) => a == b,
            (&Value::String(ref a), &Value::String(ref b)) => a == b,
            (&Value::Function(ref a), &Value::Function(ref b)) => a == b,
            (&Value::List(ref a), &Value::List(ref b)) => a == b,
            _ => false,
        }
    }
}
