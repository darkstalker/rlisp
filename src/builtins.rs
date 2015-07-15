use data::{Value, List, RuntimeError};
use scope::GlobalScope;

pub fn load(env: &mut GlobalScope)
{
    env.set_builtin("quote", false, |args, _| {
        match *args {
            List::Node(ref cons) => Ok(cons.car.clone()), //FIXME: should fail with extra args
            List::End => Err(RuntimeError::InvalidArgNum(1)),
        }
    });

    env.set_builtin("set", true, |args, env| {
        match *args {
            List::Node(ref c1) => match c1.cdr {
                List::Node(ref c2) => match c1.car {
                    Value::Symbol(ref name) => {
                        env.set(name, c2.car.clone());
                        Ok(c2.car.clone())
                    }
                    ref other => Err(RuntimeError::InvalidArgType("Symbol", other.type_name())),
                },
                List::End => Err(RuntimeError::InvalidArgNum(2)),
            },
            List::End => Err(RuntimeError::InvalidArgNum(2)),
        }
    });

    env.set_builtin("+", true, |mut args, _| {
        let mut acc = 0.0;
        while let List::Node(ref cons) = *args
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
