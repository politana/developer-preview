use std::sync::atomic::Ordering;

use politana::{Environment, Politana, is_test_suite::IS_TEST_SUITE};

use crate::{indent::Indent, test::Test, test_failure::TestFailure, webpage::Webpage};

pub async fn run_test<F: AsyncFn(Webpage) -> Result<(), TestFailure>>(test: Test<F>) {
    IS_TEST_SUITE.with(|s| s.store(true, Ordering::Relaxed));
    Politana::launch(test.view);
    let webpage = Webpage::new(Environment::document(), Environment::window());
    let result = (test.test)(webpage).await;
    if let Err(failure) = result {
        let mut log = String::new();
        log.push_str(&("=".repeat(70) + "\n"));
        log.push_str(&format!("Test failure: {}\n", test.name));
        log.push_str(&format!("File: {}\n", failure.location.file()));
        log.push_str(&format!("Location: {}:{}\n", failure.location.line(), failure.location.column()));
        log.push_str(&"Message: \n".to_string());
        log.push_str(&format!("{}\n", failure.message.indent()));
        log.push_str(&("=".repeat(70) + "\n"));
        panic!("{}", log);
    }
}
