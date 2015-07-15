use std::rc::Rc;
use std::iter::Iterator;
use data::{Value, List, Cons, Scope, RuntimeError};

impl List
{
    pub fn cons(car: Value, cdr: List) -> List
    {
        List::Node(Rc::new(Cons{ car: car, cdr: cdr }))
    }

    pub fn from_vec(mut vec: Vec<Value>) -> List
    {
        let mut cdr = List::End;
        while let Some(car) = vec.pop()
        {
            cdr = List::cons(car, cdr);
        }
        cdr
    }

    pub fn iter(&self) -> ListIter
    {
        ListIter(self)
    }

    pub fn eval(&self, env: &mut Scope) -> Result<List, RuntimeError>
    {
        let mut res = Vec::new();
        let mut iter = self.iter();
        while let Some(val) = iter.next()
        {
            res.push(try!(val.eval(env)));
        }
        Ok(List::from_vec(res))
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
