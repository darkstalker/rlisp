use std::collections::HashMap;
use std::rc::Rc;
use data::{Value, Cons, Scope, BuiltinFn, RuntimeError};
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

    pub fn set_string(&mut self, key: &str, val: &str)
    {
        self.set(key, Value::String(Rc::new(val.to_string())))
    }

    pub fn set_builtin<F>(&mut self, key: &'static str, do_eval: bool, val: F)
        where F: Fn(&Option<Rc<Cons>>, &mut Scope) -> Result<Value, RuntimeError> + 'static
    {
        self.set(key, Value::Builtin(BuiltinFn::new(key, do_eval, val)))
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
