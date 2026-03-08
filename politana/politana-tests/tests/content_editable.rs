use politana::{ContentEditable, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Content editable",
    view: || P("Hello")
        .content_editable(ContentEditable::PlaintextOnly)
        .id("editable"),
    test: async |webpage| webpage
        .element_by_id("editable")?
        .has_attribute("contenteditable", "plaintext-only")
});
