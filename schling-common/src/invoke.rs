use std::{cell::RefCell, rc::Rc};

use yew::html::ImplicitClone;

pub enum Invoke<I, O> {
    Invoke(Rc<dyn Fn(I) -> O>),
    InvokeOnce(Rc<InvokeOnce<I, O>>),
}

type InvokeOnce<I, O> = RefCell<Option<Box<dyn FnOnce(I) -> O>>>;

impl<I, O, F: Fn(I) -> O + 'static> From<F> for Invoke<I, O> {
    fn from(action: F) -> Self {
        Invoke::Invoke(Rc::new(action))
    }
}

impl<I, O> Clone for Invoke<I, O> {
    fn clone(&self) -> Self {
        match self {
            Self::Invoke(action) => Self::Invoke(action.clone()),
            Self::InvokeOnce(action) => Self::InvokeOnce(action.clone()),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<I, O> PartialEq for Invoke<I, O> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Invoke::Invoke(lhs), Invoke::Invoke(rhs)) => Rc::ptr_eq(lhs, rhs),
            (Invoke::InvokeOnce(lhs), Invoke::InvokeOnce(rhs)) => Rc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}

impl<I, O> Invoke<I, O> {
    pub fn emit(&self, value: I) -> O {
        match self {
            Invoke::Invoke(action) => action(value),
            Invoke::InvokeOnce(action) => {
                let action = action.replace(None);
                let f = action.expect("`FnOnce` callback already used");
                f(value)
            }
        }
    }

    pub fn once<F: FnOnce(I) -> O + 'static>(action: F) -> Self {
        Invoke::InvokeOnce(Rc::new(RefCell::new(Some(Box::new(action)))))
    }
}

impl<I, O: Default> Invoke<I, O> {
    pub fn noop() -> Self {
        Self::from(|_| O::default())
    }
}

impl<I, O: Default> Default for Invoke<I, O> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<I, O> ImplicitClone for Invoke<I, O> {}
