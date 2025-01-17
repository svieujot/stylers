use glob::glob;
use proc_macro2::TokenStream;
use std::fs::File;
use std::io::{self, Write};
use std::{borrow::Borrow, env::current_dir, fs};
use stylers_core::Class;
use stylers_core::{from_str, from_ts};
use syn::{Expr, Item, Stmt};

pub use stylers_macro::style;
pub use stylers_macro::style_sheet;
pub use stylers_macro::style_sheet_str;
pub use stylers_macro::style_str;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn build(output_path: Option<String>) {
    let pattern = format!("{}/src/**/*.rs", current_dir().unwrap().to_str().unwrap());
    let mut output_css = String::from("");
    p!(
        "{}",
        "===============================Stylers debug output start==============================="
    );
    for file in glob(&pattern).unwrap() {
        let file = file.unwrap();
        let content = fs::read_to_string(file).expect("Failed to read file");
        let ast = syn::parse_file(&content).unwrap();

        // check the each item in the *.rs file
        for item in ast.items {
            // check if the item is of type Function.
            if let Item::Fn(fn_def) = item {
                let _componet_name = &fn_def.sig.ident;
                // check each statement in the function
                for stmt in fn_def.block.stmts {
                    // check if any of the statment is of the form `let any_valid_variable = style!{}`
                    if let Stmt::Local(let_bin) = stmt {
                        if let Some(init) = let_bin.init {
                            if let Expr::Macro(expr_mac) = init.expr.borrow() {
                                if let Some(path_seg) = expr_mac.mac.path.segments.last() {
                                    let macro_name = path_seg.ident.clone().to_string();
                                    // p!("macro_name:{:?}", macro_name);

                                    if macro_name == String::from("style") {
                                        let ts = expr_mac.mac.tokens.clone();
                                        let class = Class::rand_class_from_seed(ts.to_string());
                                        let token_stream = TokenStream::from(ts).into_iter();
                                        let (scoped_css, _) = from_ts(token_stream, &class, false);
                                        output_css += &scoped_css;
                                    }

                                    if macro_name == String::from("style_sheet") {
                                        let ts = expr_mac.mac.tokens.clone();
                                        let file_path = ts.to_string();
                                        let file_path = file_path.trim_matches('"');
                                        let css_content = std::fs::read_to_string(&file_path)
                                            .expect("Expected to read file");

                                        let class =
                                            Class::rand_class_from_seed(css_content.to_string());
                                        let style = from_str(&css_content, &class);
                                        output_css += &style;
                                    }
                                }
                            }
                        }
                    }
                    //todo: other than let statements cover that other way style! macro can instantiated.
                }
            }
        }
    }

    write_css(output_path, &output_css)
        .unwrap_or_else(|e| p!("Problem creating output file: {}", e.to_string()));

    p!(
        "{}",
        "===============================Stylers debug output end==============================="
    );
}

const OUTPUT_DIR: &str = "./target";
/// Writes the styles in its own file and appends itself to the main.css file
fn write_css(output_path: Option<String>, content: &str) -> io::Result<()> {
    let mut out_path = String::from("./target/stylers_out.css");
    if let Some(path) = output_path {
        out_path = path;
    }

    fs::create_dir_all(&OUTPUT_DIR)?;

    let mut buffer = File::create(out_path)?;
    buffer.write_all(content.as_bytes())?;
    buffer.flush()?;

    Ok(())
}
