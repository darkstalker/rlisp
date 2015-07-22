use std::rc::Rc;
use std::iter::{Iterator, FromIterator, IntoIterator};
use data::{Value, List, Cons, Scope, RuntimeError};

impl List
{
    pub fn cons(car: Value, cdr: List) -> List
    {
        List::Node(Rc::new(Cons{ car: car, cdr: cdr }))
    }

    pub fn from_vec(vec: Vec<Value>) -> List
    {
        vec.into_iter().rev().fold(List::End, |cdr, car| List::cons(car, cdr))
    }

    pub fn iter(&self) -> ListIter
    {
        ListIter(self)
    }

    pub fn eval(&self, env: &mut Scope) -> Result<List, RuntimeError>
    {
        self.iter().map(|val| val.eval(env)).collect()
    }

    pub fn eval_to_value(&self, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        self.iter().map(|val| val.eval(env)).last().unwrap_or(Ok(Value::Nil))
    }

    pub fn call(&self, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            List::Node(ref cons) => match try!(cons.car.eval(env)) {
                Value::Function(ref func) => func.call(&cons.cdr, env, true),
                other => Err(RuntimeError::InvalidCall(other.type_name())),
            },
            List::End => Ok(Value::Nil),
        }
    }

    pub fn fold<T, F>(&self, mut acc: T, mut f: F) -> Result<T, RuntimeError>
        where F: FnMut(T, Value) -> Result<T, RuntimeError>
    {
        let mut iter = self.iter();
        while let Some(val) = iter.next()
        {
            acc = try!(f(acc, val))
        }
        Ok(acc)
    }
}

impl FromIterator<Value> for List
{
    fn from_iter<T>(iterator: T) -> Self
        where T: IntoIterator<Item=Value>
    {
        List::from_vec(iterator.into_iter().collect())
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
