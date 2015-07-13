use std::collections::HashMap;
use std::rc::Rc;
use data::{Value, Cons, Scope, BuiltinFn, RuntimeError};
use builtins;

#[derive(Debug)]
pub struct GlobalScope<'a>
{
    dict: HashMap<&'a str, Value>,
}

impl<'a> GlobalScope<'a>
{
    pub fn new() -> GlobalScope<'a>
    {
        GlobalScope{ dict: HashMap::new() }
    }

    pub fn set_number(&mut self, key: &'a str, val: f64)
    {
        self.set(key, Value::Number(val))
    }

    pub fn set_string(&mut self, key: &'a str, val: &str)
    {
        self.set(key, Value::String(val.to_string()))
    }

    pub fn set_builtin<F>(&mut self, key: &'a str, do_eval: bool, val: F)
        where F: Fn(&Option<Rc<Cons>>) -> Result<Value, RuntimeError> + 'static
    {
        self.set(key, Value::Builtin(BuiltinFn(Rc::new(val)), do_eval))
    }

    pub fn load_stdlib(&mut self)
    {
        builtins::load(self);
    }
}

impl<'a> Scope<'a> for GlobalScope<'a>
{
    fn get(&self, key: &str) -> Option<Value>
    {
        self.dict.get(key).map(|v| v.clone())
    }

    fn set(&mut self, key: &'a str, val: Value)
    {
        self.dict.insert(key, val);
    }
}
