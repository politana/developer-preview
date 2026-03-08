use std::{any::Any, marker::PhantomData, rc::Rc};

use crate::{State, utils::unwrap_or_error::UnwrapOrError};

pub struct Closure<Input, Output> {
    data: State<ClosureStorage>,
    phantom: PhantomData<(Input, Output)>
}

#[derive(Clone)]
struct ClosureStorage {
    data: Rc<dyn Any>
}

impl <Input: 'static, Output: 'static> Closure<Input, Output> {
    pub fn new(
        closure: impl Fn(Input) -> Output + 'static
    ) -> Closure<Input, Output> {
        let closure_rc = Rc::new(closure) as Rc<dyn Fn(Input) -> Output + 'static>;
        let storage = ClosureStorage {
            data: Rc::new(closure_rc)
        };
        Self {
            data: State::new(storage),
            phantom: PhantomData
        }
    }

    pub fn call(self, input: Input) -> Output {
        let data = self.data.get().data;
        // UNEXPECTED: The signatures of `new` and `call` ensure type consistency
        let closure = data.downcast_ref::<Rc<dyn Fn(Input) -> Output + 'static>>()
            .unwrap_or_unexpected();
        closure(input)
    }
}

impl <Input, Output> Clone for Closure<Input, Output> {
    fn clone(&self) -> Self { *self }
}

impl <Input, Output> Copy for Closure<Input, Output> {}
