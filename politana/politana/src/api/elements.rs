macro_rules! define_elements {
    ($($fn_name:ident $tag_str:expr)*) => {
        $(
            pub fn $fn_name(children: impl ElementChildren) -> El {
                El($tag_str, children)
            }
        )*
    };
}
pub(crate) use define_elements;

macro_rules! define_void_elements {
    ($($fn_name:ident $tag_str:expr)*) => {
        $(
            pub fn $fn_name() -> El {
                El($tag_str, ())
            }
        )*
    };
}
pub(crate) use define_void_elements;
