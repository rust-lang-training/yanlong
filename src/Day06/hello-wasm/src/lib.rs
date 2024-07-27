use wasm_bindgen::prelude::*;
// 实现效果：https://blog-huahua.oss-cn-beijing.aliyuncs.com/blog/code/rust_day6.png
// 文档：https://docs.rs/comrak/latest/comrak/
use comrak::{markdown_to_html, Options};
#[wasm_bindgen]
pub fn md_to_html(md: &str) -> String {
    markdown_to_html(md, &Options::default())
}