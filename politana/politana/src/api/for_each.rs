use std::{hash::Hash, rc::Rc};

use crate::{El, ElementChildren, api::el::ForEachContent, reactivity::hash_eq_clone::{AnyClone, HashEq}, utils::unwrap_or_error::UnwrapOrError};

pub fn ForEachKeyed<Item: Clone + 'static, Key: Hash + Eq + 'static>(
    items: impl Fn() -> Vec<Item> + 'static,
    key: impl Fn(&Item) -> Key + 'static,
    element: impl Fn(Item) -> El + 'static
) -> impl ElementChildren {
    ForEachContent {
        items: Rc::new(move || items().into_iter().map(|i| AnyClone::new(i)).collect()),
        // UNEXPECTED: item.value() is guaranteed to have type Item by the
        // function signature and correct use of ForEachContent.
        item_id: Rc::new(move |item|
            HashEq::new(key(item.value()
                .downcast_ref::<Item>().unwrap_or_unexpected()))
        ),
        // UNEXPECTED: item.value_owned() is guaranteed to have type Item by
        // the function signature and correct use of ForEachContent.
        element: Rc::new(move |item|
            element(*item.value_owned()
                .downcast::<Item>().ok().unwrap_or_unexpected())
        )
    }
}

pub fn ForEach<Item: Hash + Eq + Clone + 'static>(
    items: impl Fn() -> Vec<Item> + 'static,
    element: impl Fn(Item) -> El + 'static
) -> impl ElementChildren {
    ForEachKeyed(items, |item| item.clone(), element)
}
