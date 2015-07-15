use std::collections::HashMap;
use std::rc::Rc;
use data::{Value, List, Scope, BuiltinFn, RuntimeError};
use builtins;

#[derive(Debug)]
pub struct GlobalScope
{
    dict: HashMap<String, Value>,
}

impl GlobalScope
{
    pub fn new() -> GlobalScope
    {
        GlobalScope{ dict: HashMap::new() }
    }

    pub fn set_number(&mut self, key: &str, val: f64)
    {
        self.set(key, Value::Number(val))
    }

    pub fn set_string(&mut self, key: &str, val: String)
    {
        self.set(key, Value::String(Rc::new(val)))
    }

    pub fn set_builtin<F>(&mut self, key: &'static str, do_eval: bool, val: F)
        where F: Fn(&List, &mut Scope) -> Result<Value, RuntimeError> + 'static
    {
        self.set(key, Value::Builtin(Rc::new(BuiltinFn{ name: key, do_eval: do_eval, call: Box::new(val) })))
    }

    pub fn load_stdlib(&mut self)
    {
        self.set("nil", Value::Nil);
        builtins::load(self);
    }
}

impl Scope for GlobalScope
{
    fn get(&self, key: &str) -> Option<Value>
    {
        self.dict.get(key).map(|v| v.clone())
    }

    fn set(&mut self, key: &str, val: Value)
    {
        self.dict.insert(key.to_string(), val);
    }
}
