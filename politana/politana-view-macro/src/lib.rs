#![allow(non_snake_case)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit_mut::{self, VisitMut}, ItemFn, ExprClosure};

struct ClosureMoveVisitor;

impl VisitMut for ClosureMoveVisitor {
    fn visit_expr_closure_mut(&mut self, i: &mut ExprClosure) {
        if i.capture.is_none() {
            i.capture = Some(syn::token::Move::default());
        }
        visit_mut::visit_expr_closure_mut(self, i);
    }
}

#[proc_macro_attribute]
pub fn View(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    input_fn.attrs.push(syn::parse_quote!(#[allow(non_snake_case)]));
    let mut visitor = ClosureMoveVisitor;
    visitor.visit_item_fn_mut(&mut input_fn);
    TokenStream::from(quote!(#input_fn))
}
