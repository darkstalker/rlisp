use std::fmt;
use data::{Value, List, Function, RuntimeError};
use list::ListIter;
use scope::{Scope, RcScope};

pub struct Lambda
{
    args: Vec<String>,
    code: List,
    env: RcScope,
}

impl PartialEq for Lambda
{
    fn eq(&self, other: &Self) -> bool
    {
        self as *const _ == other as *const _
    }
}

impl fmt::Debug for Lambda
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "args:{:?} code:{} ", self.args, self.code)
    }
}

impl Lambda
{
    pub fn new(args: Vec<String>, code: List, env: RcScope) -> Lambda
    {
        Lambda{ args: args, code: code, env: env }
    }

    fn call_impl(&self, mut vals: ListIter) -> Result<Value, RuntimeError>
    {
        let mut local = Scope::local(self.env.clone());
        for (i, name) in self.args.iter().enumerate()
        {
            local.decl(&name, match vals.next() {
                Some(val) => val,
                None => return Err(RuntimeError::InvalidArgNum(self.args.len() as u32, i as u32)),
            });
        }
        self.code.eval_to_value(local.wrap())
    }
}

impl Function for Lambda
{
    fn call(&self, args: &List, env: RcScope, do_eval: bool) -> Result<Value, RuntimeError>
    {
        if do_eval
        {
            let vals = try!(args.eval(env));
            self.call_impl(vals.iter())
        }
        else
        {
            self.call_impl(args.iter())
        }
    }
}
