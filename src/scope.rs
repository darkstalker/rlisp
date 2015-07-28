use std::rc::Rc;
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use data::{Value, RuntimeError};
use builtins::{BuiltinFn, load_builtins};

pub type RcScope = Rc<RefCell<Scope>>;

pub struct Scope
{
    dict: HashMap<String, Value>,
    parent: Option<RcScope>,
}

impl Scope
{
    pub fn local(env: RcScope) -> Scope
    {
        Scope{ dict: HashMap::new(), parent: Some(env) }
    }

    pub fn global() -> Scope
    {
        Scope{ dict: HashMap::new(), parent: None }
    }

    pub fn wrap(self) -> RcScope
    {
        Rc::new(RefCell::new(self))
    }

    pub fn get(&self, key: &str) -> Option<Value>
    {
        match self.dict.get(key) {
            Some(val) => Some(val.clone()),
            None => self.parent.as_ref().and_then(|p| p.borrow().get(key)),
        }
    }

    pub fn set(&mut self, key: &str, val: Value)
    {
        if let Some(entry) = self.dict.get_mut(key)
        {
            *entry = val;
            return
        }
        match self.parent {
            Some(ref p) => p.borrow_mut().set(key, val),
            None => self.decl(key, val),
        }
    }

    pub fn decl(&mut self, key: &str, val: Value)
    {
        self.dict.insert(key.to_string(), val);
    }

    pub fn set_builtin<F>(&mut self, key: &'static str, do_eval: bool, val: F)
        where F: Fn(VecDeque<Value>, RcScope) -> Result<Value, RuntimeError> + 'static
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
