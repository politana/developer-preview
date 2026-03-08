use std::rc::Rc;

pub type Computed<Result> = Rc<dyn Fn() -> Result>;
