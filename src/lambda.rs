use data::{Value, List, Function, Scope, RuntimeError};
use list::ListIter;
use scope::LocalScope;

#[derive(Debug, PartialEq)]
pub struct Lambda
{
    args: Vec<String>,
    code: List,
    //env: &'a mut Scope,
}

impl Lambda
{
    pub fn new(args: Vec<String>, code: List) -> Lambda
    {
        Lambda{ args: args, code: code }
    }

    fn call_impl(&self, mut vals: ListIter, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        let mut local = LocalScope::new(env);
        for (i, name) in self.args.iter().enumerate()
        {
            local.decl(&name, match vals.next() {
                Some(val) => val,
                None => return Err(RuntimeError::InvalidArgNum(self.args.len() as u32, i as u32)),
            });
        }
        self.code.eval_to_value(&mut local)
    }
}

impl Function for Lambda
{
    fn call(&self, args: &List, env: &mut Scope, do_eval: bool) -> Result<Value, RuntimeError>
    {
        if do_eval
        {
            let vals = try!(args.eval(env));
            self.call_impl(vals.iter(), env)
        }
        else
        {
            self.call_impl(args.iter(), env)
        }
    }
}
