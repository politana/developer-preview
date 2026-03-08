use std::rc::Rc;

use crate::api::{computed::Computed, el::{Children, El, ForEachContent, InnerHtmlContent}};

trait IntoChild: Sized {
    fn into_child<F>(f: F) -> Children
    where F: Fn() -> Self + 'static;
}

impl IntoChild for String {
    fn into_child<F>(f: F) -> Children
    where F: Fn() -> Self + 'static {
        Children::String(Rc::new(f))
    }
}

impl IntoChild for &'static str {
    fn into_child<F>(f: F) -> Children
    where F: Fn() -> Self + 'static {
        Children::StaticString(Rc::new(f))
    }
}

impl IntoChild for InnerHtmlContent {
    fn into_child<F>(f: F) -> Children
    where F: Fn() -> Self + 'static {
        Children::InnerHtml(Rc::new(f))
    }
}

impl IntoChild for El {
    fn into_child<F>(f: F) -> Children
    where F: Fn() -> Self + 'static {
        Children::Fixed(vec![
            Rc::new(f) as Computed<El>
        ])
    }
}

pub trait ElementChildren {
    fn element_children(self) -> Children;
}

impl<F, R> ElementChildren for F
where F: Fn() -> R + 'static, R: IntoChild {
    fn element_children(self) -> Children {
        R::into_child(self)
    }
}

impl ElementChildren for () {
    fn element_children(self) -> Children {
        Children::Fixed(vec![])
    }
}

impl ElementChildren for &'static str {
    fn element_children(self) -> Children {
        (|| self.to_string()).element_children()
    }
}

impl ElementChildren for ForEachContent {
    fn element_children(self) -> Children {
        Children::ForEach(self)
    }
}

macro_rules! impl_element_children_for_tuples {
    (@do_impl $($params:ident),+) => {
        impl<$($params),*> ElementChildren for ($($params,)*)
        where
            $($params: Fn() -> El + 'static),*
        {
            fn element_children(self) -> Children {
                let ($($params,)*) = self;
                Children::Fixed(vec![
                    $( Rc::new($params) as Computed<El> ),*
                ])
            }
        }
    };
    ($head:ident, $($tail:ident),+) => {
        impl_element_children_for_tuples!(@do_impl $head, $($tail),+);
        impl_element_children_for_tuples!($($tail),+);
    };
    ($last:ident) => {
        impl_element_children_for_tuples!(@do_impl $last);
    };
}

impl_element_children_for_tuples!(
    P1,  P2,  P3,  P4,  P5,  P6,  P7,  P8,  P9,  P10,
    P11, P12, P13, P14, P15, P16, P17, P18, P19, P20,
    P21, P22, P23, P24, P25, P26, P27, P28, P29, P30,
    P31, P32, P33, P34, P35, P36, P37, P38, P39, P40,
    P41, P42, P43, P44, P45, P46, P47, P48, P49, P50,
    P51, P52, P53, P54, P55, P56, P57, P58, P59, P60,
    P61, P62, P63, P64, P65, P66, P67, P68, P69, P70,
    P71, P72, P73, P74, P75, P76, P77, P78, P79, P80,
    P81, P82, P83, P84, P85, P86, P87, P88, P89, P90,
    P91, P92, P93, P94, P95, P96, P97, P98, P99, P100
);
