use data::{Value, RuntimeError};
use scope::GlobalScope;

pub fn load(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args| {
    match *args {
        Some(ref cons) => Ok(cons.car.clone()), //FIXME: should fail with extra args
        None => Err(RuntimeError::MissingArgs),
        }
    });

    env.set_builtin("+", true, |mut args| {
        let mut acc = 0.0;
        while let Some(ref cons) = *args
        {
            match cons.car {
                Value::Number(n) => acc += n,
                _ => return Err(RuntimeError::InvalidArg),
            };
            args = &cons.cdr;
        }
        Ok(Value::Number(acc))
    });
}
