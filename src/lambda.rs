use std::fmt;
use std::collections::VecDeque;
use data::{Value, List, Function, RuntimeError};
use scope::{Scope, RcScope};

pub struct Lambda
{
    args: Vec<String>,
    code: VecDeque<Value>,
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
        write!(f, "args: {:?} code: {}", self.args,
            self.code.iter().fold(String::new(), |a, v| a + &format!("{} ", v)))
    }
}

impl Lambda
{
    pub fn new(args: Vec<String>, code: VecDeque<Value>, env: RcScope) -> Lambda
    {
        Lambda{ args: args, code: code, env: env }
    }
}

impl Function for Lambda
{
    fn call(&self, args: &List, env: RcScope, do_eval: bool) -> Result<Value, RuntimeError>
    {
        let vals = if do_eval { try!(args.eval(env)) } else { args.iter().collect() };
        let (na, nv) = (self.args.len(), vals.len());
        if nv < na { return Err(RuntimeError::InvalidArgNum(na as u32, nv as u32)) }

        let mut local = Scope::local(self.env.clone());
        for (name, val) in self.args.iter().zip(vals.into_iter())
        {
            local.decl(&name, val);
        }

        let wenv = local.wrap();
        let mut last = Value::Nil;
        for val in self.code.iter()
        {
            last = try!(val.eval(wenv.clone()));
        }
        Ok(last)
    }
}
