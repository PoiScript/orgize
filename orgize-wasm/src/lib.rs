use orgize::{rowan::ast::AstNode, Org as Inner};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Org {
    inner: Inner,
}

#[wasm_bindgen]
impl Org {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Self {
        Org {
            inner: Inner::parse(input),
        }
    }

    pub fn html(&self) -> String {
        self.inner.to_html()
    }

    pub fn org(&self) -> String {
        self.inner.to_org()
    }

    pub fn syntax(&self) -> String {
        format!("{:#?}", self.inner.document().syntax())
    }

    pub fn update(&mut self, s: &str) {
        self.inner = Inner::parse(s);
    }

    #[wasm_bindgen(getter, js_name = "buildTime")]
    pub fn build_time() -> String {
        env!("CARGO_BUILD_TIME").into()
    }

    #[wasm_bindgen(getter, js_name = "gitHash")]
    pub fn git_hash() -> String {
        env!("CARGO_GIT_HASH").into()
    }
}
