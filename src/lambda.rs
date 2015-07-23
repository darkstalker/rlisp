use data::{Value, List, Function, Scope, RuntimeError};
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
}

impl Function for Lambda
{
    fn call(&self, args: &List, env: &mut Scope, _: bool) -> Result<Value, RuntimeError>
    {
        let mut local = LocalScope::new(env);
        let mut vals = args.iter();
        for name in self.args.iter()
        {
            local.decl(&name, vals.next().unwrap_or(Value::Nil));
        }
        self.code.eval_to_value(&mut local)
    }
}
