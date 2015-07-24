use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use data::{Value, List, Scope, RcScope, RuntimeError};
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

    pub fn wrap(self) -> Rc<RefCell<GlobalScope>>
    {
        Rc::new(RefCell::new(self))
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
        where F: Fn(&List, RcScope) -> Result<Value, RuntimeError> + 'static
    {
        self.set(key, Value::Builtin(Rc::new(BuiltinFn{ name: key, do_eval: do_eval, func: Box::new(val) })))
    }

    pub fn load_stdlib(&mut self)
    {
        self.set("nil", Value::Nil);
        self.set("#t", Value::Bool(true));
        self.set("#f", Value::Bool(false));
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

pub struct LocalScope
{
    dict: HashMap<String, Value>,
    parent: RcScope,
}

impl LocalScope
{
    pub fn new(env: RcScope) -> LocalScope
    {
        LocalScope{ dict: HashMap::new(), parent: env }
    }

    pub fn wrap(self) -> Rc<RefCell<LocalScope>>
    {
        Rc::new(RefCell::new(self))
    }
}

impl Scope for LocalScope
{
    fn get(&self, key: &str) -> Option<Value>
    {
        match self.dict.get(key) {
            Some(val) => Some(val.clone()),
            None => self.parent.borrow().get(key),
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
            self.parent.borrow_mut().set(key, val)
        }
    }

    fn decl(&mut self, key: &str, val: Value)
    {
        self.dict.insert(key.to_string(), val);
    }
}
