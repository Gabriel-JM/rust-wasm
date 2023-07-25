use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, testing!");
}

#[wasm_bindgen]
pub fn add_heading() -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("No Window")
        .document()
        .expect("No Document")
    ;
    let body = document.body().expect("No Body");

    let heading = document.create_element("h1")?;
    heading.set_inner_html("This heading was created from Rust!");

    body.append_child(&heading)?;

    Ok(())
}
