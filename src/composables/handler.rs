use std::rc::Rc;

pub struct Handler<T>(Rc<dyn Fn(T)>);

impl<T> Handler<T> {
    pub fn call(&self, arg: T) {
        (self.0)(arg);
    }
}

impl<T> Clone for Handler<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Default for Handler<T> {
    fn default() -> Self {
        Self(Rc::new(|_| {}))
    }
}

impl<F, T> From<F> for Handler<T>
where
    F: Fn(T) + 'static,
{
    fn from(value: F) -> Self {
        Self(Rc::new(value))
    }
}
