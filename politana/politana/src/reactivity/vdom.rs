use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc, sync::atomic::Ordering};

use web_sys::{Document, HtmlElement};

use crate::{El, api::{computed::Computed, el::{ForEachContent, InnerHtmlContent}, property::{Property, PropertyKey}}, debug_bar::inspection::tree::InspectorElement, reactivity::{deconstruct::DID_DECONSTRUCT, generate_inspector_tree::generate_inspector_tree, hash_eq_clone::HashEq, style_manager::{StyleManager, StyleSpec}, vdom_ref::VdomRef}, utils::{error_messages, report_error::report_error, unwrap_or_error::UnwrapOrError}};

pub struct Vdom {
    pub html: HtmlElement,
    pub virtual_el: Option<VirtualElement>,
    pub nav_prefix: Vec<Rc<String>>,
    pub properties: Vec<VdomRef>,
    /// Resources to be dropped BEFORE the HTML element is removed
    pub resources_drop_before: Vec<Box<dyn Any>>,
    /// Resources to be dropped AFTER the HTML element is removed
    pub resources_drop_after: Vec<Box<dyn Any>>,
    /// The number of parents the VDOM has. Used as a heuristic to track bad state reads
    pub nesting_level: u64
}

pub enum VirtualElement {
    ReplaceElement {
        current_children: Vec<VdomRef>,
        content: Computed<El>
    },
    ForEach {
        current_children: HashMap<HashEq, VdomRef>,
        content: ForEachContent
    },
    UpdateProperty {
        key: PropertyKey,
        value: Property,
        current_style: Option<StyleSpec>
    },
    UpdateClassList {
        computed: Vec<Computed<Vec<String>>>,
        current: Vec<String>
    },
    StaticString(Computed<&'static str>),
    String(Computed<String>),
    InnerHtml(Computed<InnerHtmlContent>),
    Effect {
        closure: Rc<dyn Fn() -> bool>,
        /// An ephemeral reference
        last_run_dependencies: VdomRef
    }
}

thread_local! {
    static DOCUMENT: RefCell<Option<Document>> = Default::default();
    /// Reflects the structure of the document
    static VDOM_TREE: RefCell<VdomRef> = RefCell::new(VdomRef::null());
    /// The last element owns all created resources
    static VDOM_STACK: RefCell<Vec<VdomRef>> = Default::default();
    /// If non-zero, Vdom::current() returns None
    static INERT_STACK: RefCell<Vec<bool>> = RefCell::new(vec![false]);
}

impl Vdom {
    pub fn new(
        html: HtmlElement,
        virtual_el: VirtualElement,
        nav_prefix: Vec<Rc<String>>
    ) -> Self {
        let current_nesting_level = Vdom::current()
            .map(|v| v.map(|v| v.nesting_level)).flatten().unwrap_or(0);
        Self {
            html,
            virtual_el: Some(virtual_el),
            nav_prefix,
            properties: Vec::new(),
            resources_drop_before: Vec::new(),
            resources_drop_after: Vec::new(),
            nesting_level: current_nesting_level + 1
        }
    }

    pub fn init_tree(document: Document, root: VdomRef) {
        DOCUMENT.set(Some(document));
        VDOM_TREE.set(root);
    }

    pub fn using<T>(r: VdomRef, closure: impl FnOnce() -> T) -> T {
        VDOM_STACK.with_borrow_mut(|s| s.push(r));
        let result = closure();
        VDOM_STACK.with_borrow_mut(|s| s.pop());
        result
    }

    pub fn inert<T>(inert: bool, closure: impl FnOnce() -> T) {
        if DID_DECONSTRUCT.load(Ordering::Relaxed) {
            // After deconstruction, don't call callbacks for on_disappear or related
            return;
        }
        INERT_STACK.with_borrow_mut(|s| s.push(inert));
        closure();
        INERT_STACK.with_borrow_mut(|s| s.pop());
    }

    pub fn current() -> Option<VdomRef> {
        if INERT_STACK.with_borrow(|s| s.last().copied()).unwrap_or_unexpected() {
            return None;
        }
        VDOM_STACK.with_borrow(|s| s.last().copied())
    }

    pub fn document() -> Document {
        DOCUMENT.with_borrow(|d| d.clone()
            .unwrap_or_else(|| report_error(error_messages::UNEXPECTED)))
    }

    pub fn inspector_tree() -> InspectorElement {
        generate_inspector_tree(VDOM_TREE.with_borrow(|t| *t))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn nav_prefix(&self) -> Vec<Rc<String>> {
        self.nav_prefix.clone()
    }
}

impl Drop for Vdom {
    fn drop(&mut self) {
        match &self.virtual_el {
            Some(VirtualElement::ReplaceElement { current_children, .. }) => {
                for r in current_children {
                    r.destroy();
                }
            }
            Some(VirtualElement::ForEach { current_children, .. }) => {
                for (_, r) in current_children {
                    r.destroy();
                }
            }
            Some(VirtualElement::UpdateProperty { current_style, .. }) => {
                current_style.as_ref().map(StyleManager::release_style);
            }
            Some(VirtualElement::UpdateClassList { .. }) => {}
            Some(VirtualElement::StaticString(_)) => {}
            Some(VirtualElement::String(_)) => {}
            Some(VirtualElement::InnerHtml(_)) => {}
            Some(VirtualElement::Effect { last_run_dependencies, .. }) => {
                last_run_dependencies.destroy();
            }
            None => {}
        }
        for property in &self.properties {
            property.destroy();
        }
    }
}
