use crate::utils::parse_group;
use proc_macro2::Group;

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
#[derive(Debug)]
pub struct CSSStyleDeclaration{
    //e.g {color:red;}
    style_css_text: String
}

impl CSSStyleDeclaration {
    pub fn parse(group: Group)->CSSStyleDeclaration{
        CSSStyleDeclaration { style_css_text:parse_group(group) }
    }
    pub fn style_css_text(&self)->String{
        self.style_css_text.clone()
    }
}