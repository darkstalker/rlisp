use data::Value;
use data::RuntimeError::*;
use scope::GlobalScope;

pub fn load(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args, _| {
        args.iter().next().ok_or(InvalidArgNum(1))
    });

    env.set_builtin("set", true, |args, env| {
        let mut iter = args.iter();
        let key = try!(iter.next().ok_or(InvalidArgNum(2)));
        let val = try!(iter.next().ok_or(InvalidArgNum(2)));
        match key {
            Value::Symbol(name) => {
                env.set(&name, val.clone());
                Ok(val)
            },
            other => Err(InvalidArgType("Symbol", other.type_name())),
        }
    });

    env.set_builtin("+", true, |args, _| {
        let mut acc = 0.0;
        let mut iter = args.iter();
        while let Some(val) = iter.next()
        {
            acc += match val {
                Value::Number(n) => n,
                other => return Err(InvalidArgType("Number", other.type_name())),
            };
        }
        Ok(Value::Number(acc))
    });
}
