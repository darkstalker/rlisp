use std::fmt;
use data::{Value, List, Function, Scope, RcScope, RuntimeError};
use list::ListIter;
use scope::LocalScope;

pub struct Lambda
{
    args: Vec<String>,
    code: List,
    env: RcScope,
}

//FIXME: not correct
impl PartialEq for Lambda
{
    fn eq(&self, other: &Self) -> bool
    {
        self.args == other.args && self.code == other.code
    }
}

impl fmt::Debug for Lambda
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Lambda{{ args: {:?}, code: {:?}}}", self.args, self.code)
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
        let mut local = LocalScope::new(self.env.clone());
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
