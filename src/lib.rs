// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_wasm_dodrio_templating
//!
//! **html templating for dodrio**  
//! ***[repo](https://github.com/LucianoBestia/rust_wasm_dodrio_templating); version: 1.0.3  date: 2021-01-13 authors: Luciano Bestia***  
//!
//!  [![crates.io](https://meritbadge.herokuapp.com/rust_wasm_dodrio_templating)](https://crates.io/crates/rust_wasm_dodrio_templating) [![Documentation](https://docs.rs/rust_wasm_dodrio_templating/badge.svg)](https://docs.rs/rust_wasm_dodrio_templating/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/rust_wasm_dodrio_templating.svg)](https://web.crev.dev/rust-reviews/crate/rust_wasm_dodrio_templating/) [![RustActions](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/workflows/rust/badge.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/) [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://lucianobestia.github.io/rust_wasm_dodrio_templating/rust_wasm_dodrio_templating/index.html) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/blob/master/LICENSE)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-261-green.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-145-blue.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-51-purple.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_wasm_dodrio_templating/)
//!
//! ## Html templating
//!
//! In the past I wrote html inside Rust code with the macro `html!` from the `crate typed-html`  
//! <https://github.com/bodil/typed-html>  
//! It has also a macro `dodrio !` created exclusively for the dodrio vdom.  
//! I had two main problems with this approach:  
//!
//! 1. Any change to the html required a recompiling. And that is very slow in Rust.  
//! 2. I could not add new html elements, that the macro don't recognize. I wanted to use SVG. There was not support for that.  
//!
//! I reinvented the wheel - "html templating".  
//! First a graphical designer makes a html/css page that looks nice. No javascript, nothing is dynamic. It is just a graphical template.  
//! Then I insert in it html comments and "data-" attributes that I can later replace in my code.  
//! The html is not changed graphically because of it. So both the graphical designer and the programmer are still happy.  
//! In my code I parse the html template as a microXml file. Basically they are the same with small effort. When I find a comment or "data-" attribute then the value of the next node is replaced.  
//! I can replace attributes, strings and entire nodes. And I can insert event for behavior with "data-wt".  
//! When developing, the html template is loaded and parsed and a dodrio node is created. That is not very fast. But I can change the html in real time and see it rendered without compiling the Rust code. This is super efficient for development.  
//! I have in plans to add a Rust code generator, that creates the Rust code for the dodrio node before compile time. In that case nothing is parsed in runtime and I expect great speeds. But the flexibility of easily changing the html template is gone. For every change I must recompile the Rust code.  
//!
//! ## Used in projects
//!
//! <https://github.com/LucianoBestia/unforgettable7_game/>  
//!
//! ## How to use it
//!
//! Inside a perfectly working static html insert special comments and attributes to replace the next node or attribute.  
//!
//! ### Replace the next text node  
//!
//! Insert a comment that starts with "wt_" (webbrowser text).  
//! After that is the name of the enum to replace in the fn replace_with_string().  
//!
//! ```html
//! <p><!--wt_new_text>old_text</p>
//! ```
//!
//! ### Replace the next node with nodes  
//!
//! Insert a comment that starts with "wn_" (webbrowser nodes).  
//! After that is the name of the enum to replace in the fn replace_with_nodes().  
//! The computed nodes can be complicated with a lot of html. If needed, this fragments of html are saved inside the html template as sub_templates.  
//!
//! ```html
//! <p><!--wn_new_nodes><div id="old_node">...</div></p>
//! ```
//!
//! ### Replace the next attribute text value  
//!
//! Insert an attribute that starts with "data-wt-" (webbrowser text).  
//! The attribute name finishes in the name of the next attribute.  
//! The attribute value is the enum to use in the fn replace_with_string().  
//!
//! ```html
//! <input data-wt-value="wt_new_text" value="old text" />
//! ```
//!
//! ### Set the event handler  
//!
//! Insert an attribute that starts with "data-on-".  
//! The attribute name finishes in the name of the event to handle.  
//! The attribute value is the enum to use in the fn set_event_listener().  
//! The enum name must start with "wl_" (Webbrowser listener).
//!
//! ```html
//! <input data-on-keyup="wl_nickname_on_keyup" />
//! ```
//!
//! ### Sub_templates
//!
//! When a part of the html template needs to be repeated, we use sub_templates.
//! A sub_template is inside the html template in the node `<template>`.  
//!
//! ```html
//! <template name="sub_template_name">
//!         <div>some html</div>
//! </template>
//! ```
//!
//! The sub_template has a name attribute that is used for replacement in Rust code to return a vector of nodes for replace the "wn_" special comment.  
//!
//! ```ignore
//! pub fn div_grid_all_items<'a>(
//!     rrc: &RootRenderingComponent,
//!     cx: &mut RenderContext<'a>,
//! ) -> Vec<Node<'a>> {
//!     let mut vec_grid_items: Vec<Node<'a>> = Vec::new();
//!     for x in 1..=10 {
//!         let html_template = rrc.web_data.get_sub_template("sub_template_name");
//!
//!         let grid_item = rrc.render_template(
//!             cx,
//!             &html_template,
//!             rust_wasm_dodrio_templating::html_template_mod::HtmlOrSvg::Html
//!         ).unwrap();
//!         vec_grid_items.push(grid_item_bump);
//!     }
//!
//!     // return
//!     vec_grid_items
//! }
//! ```
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

pub mod html_template_mod;
