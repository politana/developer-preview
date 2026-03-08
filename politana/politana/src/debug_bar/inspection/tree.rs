use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectorElement {
    pub tag: String,
    pub children: InspectorChildren
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InspectorChildren {
    Children(Vec<InspectorElement>),
    String(String),
    InnerHtml(String),
    Unexpected
}
