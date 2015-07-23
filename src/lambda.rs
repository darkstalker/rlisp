use std::hash::{Hash, Hasher, SipHasher};
use data::{Value, List, Function, Scope, RuntimeError};
use scope::LocalScope;

pub struct Lambda
{
    name: String,
    args: Vec<String>,
    code: List,
    //env: &'a mut Scope,
}

impl Lambda
{
    pub fn new(args: Vec<String>, code: List) -> Lambda
    {
        let mut hasher = SipHasher::new();
        args.hash(&mut hasher);
        format!("{}", code).hash(&mut hasher);
        let name = format!("<lambda:{:x}>", hasher.finish());

        Lambda{ name: name, args: args, code: code }
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

    fn get_name(&self) -> &str
    {
        &self.name
    }
}
