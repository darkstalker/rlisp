use std::collections::HashMap;
use std::rc::Rc;
use data::{Value, List, Scope, RuntimeError};
use builtins::{BuiltinFn, load_builtins};

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
        self.set(key, Value::Builtin(Rc::new(BuiltinFn{ name: key, do_eval: do_eval, func: Box::new(val) })))
    }

    pub fn load_stdlib(&mut self)
    {
        self.set("nil", Value::Nil);
        self.set("true", Value::Bool(true));
        self.set("false", Value::Bool(false));
        load_builtins(self);
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

    fn decl(&mut self, key: &str, val: Value)
    {
        self.set(key, val)
    }
}

pub struct LocalScope<'a>
{
    dict: HashMap<String, Value>,
    parent: &'a mut Scope,
}

impl<'a> LocalScope<'a>
{
    pub fn new(env: &mut Scope) -> LocalScope
    {
        LocalScope{ dict: HashMap::new(), parent: env }
    }
}

impl<'a> Scope for LocalScope<'a>
{
    fn get(&self, key: &str) -> Option<Value>
    {
        match self.dict.get(key) {
            Some(val) => Some(val.clone()),
            None => self.parent.get(key),
        }
    }

    fn set(&mut self, key: &str, val: Value)
    {
        if let Some(entry) = self.dict.get_mut(key)
        {
            *entry = val;
        }
        else
        {
            self.parent.set(key, val)
        }
    }

    fn decl(&mut self, key: &str, val: Value)
    {
        self.dict.insert(key.to_string(), val);
    }
}
