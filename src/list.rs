use std::rc::Rc;
use std::collections::VecDeque;
use std::iter::{Iterator, FromIterator, IntoIterator, DoubleEndedIterator};
use data::{Value, List, Function, Cons, RuntimeError};
use scope::RcScope;

impl List
{
    pub fn cons(car: Value, cdr: List) -> List
    {
        List::Node(Rc::new(Cons{ car: car, cdr: cdr }))
    }

    pub fn from_de_iter<I>(iter: I) -> List
        where I: DoubleEndedIterator<Item=Value>
    {
        iter.rev().fold(List::End, |cdr, car| List::cons(car, cdr))
    }

    pub fn iter(&self) -> ListIter
    {
        ListIter(self)
    }

    pub fn eval(&self, env: RcScope) -> Result<VecDeque<Value>, RuntimeError>
    {
        self.iter().map(|val| val.eval(env.clone())).collect()
    }

    pub fn call(&self, env: RcScope) -> Result<Value, RuntimeError>
    {
        match *self {
            List::Node(ref cons) => match try!(cons.car.eval(env.clone())) {
                Value::Builtin(ref func) => func.call(&cons.cdr, env, true),
                Value::Lambda(ref func) => func.call(&cons.cdr, env, true),
                other => Err(RuntimeError::InvalidCall(other.type_name())),
            },
            List::End => Ok(Value::Nil),
        }
    }
}

impl FromIterator<Value> for List
{
    fn from_iter<T>(iterator: T) -> Self
        where T: IntoIterator<Item=Value>
    {
        List::from_de_iter(iterator.into_iter().collect::<Vec<_>>().into_iter())    //ugly
    }
}

#[derive(Clone)]
pub struct ListIter<'a>(&'a List);

impl<'a> Iterator for ListIter<'a>
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item>
    {
        match *self.0 {
            List::Node(ref cons) => {
                self.0 = &cons.cdr;
                Some(cons.car.clone())
            },
            List::End => None,
        }
    }
}

pub fn fold_result<I, T, F>(mut iter: I, mut acc: T, mut f: F) -> Result<T, RuntimeError>
    where F: FnMut(T, Value) -> Result<T, RuntimeError>, I: Iterator<Item=Value>
{
    while let Some(val) = iter.next()
    {
        acc = try!(f(acc, val))
    }
    Ok(acc)
}
