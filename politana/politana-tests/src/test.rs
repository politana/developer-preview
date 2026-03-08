use politana::El;

use crate::{test_failure::TestFailure, webpage::Webpage};

pub struct Test<F: AsyncFn(Webpage) -> Result<(), TestFailure>> {
    pub name: &'static str,
    pub view: fn() -> El,
    pub test: F
}

#[macro_export]
macro_rules! politana_test {
    ($test_body:expr) => {
        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn politana_test() {
            politana_tests::run_test::run_test($test_body).await;
        }
    };
}
