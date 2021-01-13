//! **html_template_mod**  
//! Html templating for dodrio, generic code for a standalone library.
//! The implementation is in another file where RootRenderingComponents
//! implement the trait HtmlTemplating

// region: use
use reader_for_microxml::*;

use dodrio::{
    builder::{text, ElementBuilder},
    bumpalo::{self},
    Attribute, Listener, Node, RenderContext, RootRender, VdomWeak,
};
use unwrap::unwrap;
// endregion: use

/// Svg elements are different because they have a namespace
#[derive(Clone, Copy)]
pub enum HtmlOrSvg {
    /// html element
    Html,
    /// svg element
    Svg,
}

/// the RootRenderingComponent struct must implement this trait
/// it must have the fields for local_route and html_template fields
pub trait HtmlTemplating {
    // region: methods to be implemented for a specific project
    // while rendering, cannot mut rrc
    fn replace_with_string(&self, fn_name: &str) -> String;
    fn retain_next_node_or_attribute<'a>(&self, fn_name: &str) -> bool;
    fn replace_with_nodes<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Vec<Node<'a>>;
    fn set_event_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, VdomWeak, web_sys::Event) + 'static>;
    // endregion: methods to be implemented

    // region: generic code (in trait definition)

    /// get root element Node.   
    /// I wanted to use dodrio::Node, but it has only private methods.  
    /// I must use dodrio element_builder.  
    fn render_template<'a>(
        &self,
        cx: &mut RenderContext<'a>,
        html_template: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Node<'a>, String> {
        let mut reader_for_microxml = ReaderForMicroXml::new(html_template);
        let mut dom_path = Vec::new();
        let mut root_element;
        let mut html_or_svg_local = html_or_svg_parent;
        let bump = cx.bump;
        // the first element must be root and is special
        #[allow(clippy::single_match_else, clippy::wildcard_enum_match_arm)]
        match reader_for_microxml.next() {
            None => {
                // return error
                return Err("Error: no root element".to_owned());
            }
            Some(result_token) => {
                match result_token {
                    Result::Err(e) => {
                        // return error
                        return Err(format!("Error: {}", e));
                    }
                    Result::Ok(token) => {
                        match token {
                            Token::StartElement(name) => {
                                dom_path.push(name.to_owned());
                                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                                root_element = ElementBuilder::new(bump, name);
                                if name == "svg" {
                                    html_or_svg_local = HtmlOrSvg::Svg;
                                }
                                if let HtmlOrSvg::Svg = html_or_svg_local {
                                    // svg elements have this namespace
                                    root_element =
                                        root_element.namespace(Some("http://www.w3.org/2000/svg"));
                                }
                                // recursive function can return error
                                match self.fill_element_builder(
                                    &mut reader_for_microxml,
                                    root_element,
                                    cx,
                                    html_or_svg_local,
                                    &mut dom_path,
                                ) {
                                    // the methods are move, so I have to return the moved value
                                    Ok(new_root_element) => root_element = new_root_element,
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                            _ => {
                                // return error
                                return Err("Error: no root element".to_owned());
                            }
                        }
                    }
                }
            }
        }
        // return
        Ok(root_element.finish())
    }
    /// Recursive function to fill the Element with attributes and sub-nodes(Element, Text, Comment).  
    /// Moves & Returns ElementBuilder or error.  
    /// I must `move` ElementBuilder because its methods are all `move`.  
    /// It makes the code less readable. It is only good for chaining and type changing.  
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_builder<'a>(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementBuilder<
            'a,
            bumpalo::collections::Vec<'a, Listener<'a>>,
            bumpalo::collections::Vec<'a, Attribute<'a>>,
            bumpalo::collections::Vec<'a, Node<'a>>,
        >,
        cx: &mut RenderContext<'a>,
        html_or_svg_parent: HtmlOrSvg,
        dom_path: &mut Vec<String>,
    ) -> Result<
        ElementBuilder<
            'a,
            bumpalo::collections::Vec<'a, Listener<'a>>,
            bumpalo::collections::Vec<'a, Attribute<'a>>,
            bumpalo::collections::Vec<'a, Node<'a>>,
        >,
        String,
    > {
        let mut replace_string: Option<String> = None;
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        let bump = cx.bump;
        // loop through all the siblings in this iteration
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.next() {
                None => {}
                Some(result_token) => {
                    match result_token {
                        Result::Err(e) => {
                            // return error
                            return Err(format!("Error: {}", e));
                        }
                        Result::Ok(token) => {
                            match token {
                                Token::StartElement(name) => {
                                    dom_path.push(name.to_owned());
                                    // construct a child element and fill it (recursive)
                                    let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                                    let mut child_element = ElementBuilder::new(bump, name);
                                    if name == "svg" {
                                        // this tagname changes to svg now
                                        html_or_svg_local = HtmlOrSvg::Svg;
                                    }
                                    if let HtmlOrSvg::Svg = html_or_svg_local {
                                        // this is the
                                        // svg elements have this namespace
                                        child_element = child_element
                                            .namespace(Some("http://www.w3.org/2000/svg"));
                                    }
                                    if name == "foreignObject" {
                                        // this tagname changes to html for children, not for this element
                                        html_or_svg_local = HtmlOrSvg::Html;
                                    }
                                    child_element = self.fill_element_builder(
                                        reader_for_microxml,
                                        child_element,
                                        cx,
                                        html_or_svg_local,
                                        dom_path,
                                    )?;
                                    // if the boolean is empty or true then render the next node
                                    if replace_boolean.unwrap_or(true) {
                                        if let Some(repl_vec_nodes) = replace_vec_nodes {
                                            for repl_node in repl_vec_nodes {
                                                element = element.child(repl_node);
                                            }
                                            replace_vec_nodes = None;
                                        } else {
                                            element = element.child(child_element.finish());
                                        }
                                    }
                                    if replace_boolean.is_some() {
                                        replace_boolean = None;
                                    }
                                }
                                Token::Attribute(name, value) => {
                                    if name.starts_with("data-wt-") {
                                        // the rest of the name does not matter,
                                        // but it should be nice to be te name of the next attribute.
                                        // The replace_string will always be applied to the next attribute.
                                        let fn_name = value;
                                        if &fn_name[..3] != "wt_" {
                                            return Err(format!(
                                                "{} value does not start with wt_ : {}.",
                                                name, fn_name
                                            ));
                                        }
                                        let repl_txt = self.replace_with_string(fn_name);
                                        replace_string = Some(repl_txt);
                                    } else if name.starts_with("data-on-") {
                                        // it must look like data-on-click="wl_xxx" wl_ = webbrowser listener
                                        // Only one listener for now because the api does not give me other method.
                                        let fn_name = value.to_string();
                                        let event_to_listen = unwrap!(name.get(8..)).to_string();
                                        // rust_wasm_websys_utils::websysmod::debug_write(&format!("name.starts_with data-on- : .{}.{}.",&fn_name,&event_to_listen));
                                        if !fn_name.is_empty() && &fn_name[..3] != "wl_" {
                                            return Err(format!(
                                                "{} value does not start with wl_ : {}.",
                                                name, fn_name
                                            ));
                                        }

                                        let event_to_listen =
                                            bumpalo::format!(in bump, "{}",&event_to_listen)
                                                .into_bump_str();
                                        element = element
                                            .on(event_to_listen, self.set_event_listener(fn_name));
                                    } else {
                                        let name =
                                            bumpalo::format!(in bump, "{}",name).into_bump_str();
                                        let value2;
                                        if let Some(repl) = replace_string {
                                            value2 =
                                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                                                .into_bump_str();
                                            // empty the replace_string for the next node
                                            replace_string = None;
                                        } else {
                                            value2 =
                                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(value))
                                                .into_bump_str();
                                        }
                                        element = element.attr(name, value2);
                                    }
                                }
                                Token::TextNode(txt) => {
                                    let txt2;
                                    if let Some(repl) = replace_string {
                                        txt2 =
                                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                                                .into_bump_str();
                                        // empty the replace_string for the next node
                                        replace_string = None;
                                    } else {
                                        txt2 = bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(txt))
                                            .into_bump_str();
                                    }
                                    // here accepts only utf-8.
                                    // rust_wasm_websys_utils::websysmod::debug_write("text node");
                                    // rust_wasm_websys_utils::websysmod::debug_write(txt2);
                                    // only minimum html entities are decoded
                                    element = element.child(text(txt2));
                                }
                                Token::Comment(txt) => {
                                    // the main goal of comments is to change the value of the next text node
                                    // with the result of a function
                                    if txt == "end_of_wt" {
                                        // a special comment <!--end_of_wt--> just to end the wt_ replace string
                                        // if there are more replacing inside one text node
                                        // rust_wasm_websys_utils::websysmod::debug_write("found comment <!--end_of_wt-->");
                                    } else if txt.starts_with("wt_") {
                                        // it must look like <!--wt_get_text-->  wt_ = webbrowser text
                                        let repl_txt = self.replace_with_string(txt);
                                        replace_string = Some(repl_txt);
                                    } else if txt.starts_with("wn_") {
                                        // it must look like <!--wn_get_nodes-->  wn_ = webbrowser nodes
                                        let repl_vec_nodes = self.replace_with_nodes(cx, txt);
                                        replace_vec_nodes = Some(repl_vec_nodes);
                                    } else if txt.starts_with("wb_") {
                                        // it must look like <!--wb_get_bool-->  wb_ = webbrowser boolean
                                        // boolean if this is true than render the next node, else don't render
                                        replace_boolean =
                                            Some(self.retain_next_node_or_attribute(txt));
                                    } else {
                                        // nothing. it is really a comment
                                    }
                                }
                                Token::EndElement(name) => {
                                    let last_name = unwrap!(dom_path.pop());
                                    // it can be also auto-closing element
                                    if last_name == name || name == "" {
                                        return Ok(element);
                                    } else {
                                        return Err(format!(
                                            "End element not correct: starts <{}> ends </{}>",
                                            last_name, name
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // endregion: generic code
}

/// get en empty div node
pub fn empty_div<'a>(cx: &mut RenderContext<'a>) -> Node<'a> {
    let bump = cx.bump;
    ElementBuilder::new(bump, "div").finish()
}

/// decode 5 xml control characters : " ' & < >  
/// https://www.liquid-technologies.com/XML/EscapingData.aspx
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// https://www.tutorialspoint.com/html5/html5_entities.htm  
pub fn decode_5_xml_control_characters(input: &str) -> String {
    // The standard library replace() function makes allocation,
    //but is probably fast enough for my use case.
    input
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}
