use data::{Value, RuntimeError};
use scope::GlobalScope;

pub fn load(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args| {
        match *args {
            Some(ref cons) => Ok(cons.car.clone()), //FIXME: should fail with extra args
            None => Err(RuntimeError::InvalidArgNum(1)),
        }
    });

    env.set_builtin("+", true, |mut args| {
        let mut acc = 0.0;
        while let Some(ref cons) = *args
        {
            match cons.car {
                Value::Number(n) => acc += n,
                ref other => return Err(RuntimeError::InvalidArgType("Number", other.type_name())),
            };
            args = &cons.cdr;
        }
        Ok(Value::Number(acc))
    });
}
