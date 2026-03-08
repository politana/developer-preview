use politana::P;
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Hello World",
    view: || P("Hello world!").id("hi"),
    test: async |webpage| webpage.element_by_id("hi")?.has_text("Hello world!")
});
