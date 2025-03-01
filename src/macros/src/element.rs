extern crate proc_macro2;
extern crate quote;
extern crate syn;

use self::proc_macro2::TokenStream;
use self::quote::quote;
use self::syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
    punctuated::Punctuated,
    FnArg, ItemFn, LitStr, Pat, Token,
};

struct ElementAttributes {
    emt_type: LitStr,
}

impl Parse for ElementAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ElementAttributes {
            emt_type: input.parse()?,
        })
    }
}

fn get_function_parameter_names(inputs: &Punctuated<FnArg, Token![,]>) -> Vec<String> {
    inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                // Match named arguments (e.g., `x: i32`)
                FnArg::Typed(pat_type) => {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some(pat_ident.ident.to_string())
                    } else {
                        None
                    }
                }
                // Ignore self arguments (e.g., `self`, `&self`, `&mut self`)
                FnArg::Receiver(_) => None,
            }
        })
        .collect()
}

fn remove_function_parameter(
    param_name: &str,
    inputs: &mut Punctuated<FnArg, Token![,]>,
) -> Option<FnArg> {
    let mut ret = None;
    let mut new_inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
    for arg in inputs.iter() {
        match arg {
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                    if pat_ident.ident == param_name {
                        ret = Some(arg.clone());
                    } else {
                        new_inputs.push(arg.clone());
                    }
                }
            }
            FnArg::Receiver(_) => {}
        }
    }
    *inputs = new_inputs;
    ret
}

pub fn macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let out_attrs: ElementAttributes = parse2(attr).unwrap();
    let out_items: ItemFn = parse2(item).unwrap();

    let fn_name = &out_items.sig.ident;
    let fn_args = &out_items.sig.inputs;
    let fn_body = &out_items.block;

    let emt_type = out_attrs.emt_type;
    let emt_name = fn_name.to_string().to_lowercase();

    let mut new_fn_args = fn_args.to_owned();

    let removed_style = remove_function_parameter("style", &mut new_fn_args);
    let removed_icons = remove_function_parameter("icons", &mut new_fn_args);

    let current_dir = quote! {
        fn current_dir(theme_dir: &std::path::PathBuf) -> std::path::PathBuf {
           theme_dir
               .join(format!("{}s", #emt_type))
               .join(#emt_name)
        }
    };

    let mut style = TokenStream::new();

    if removed_style.is_some() {
        style = quote! {
            let style: String = {
                use crate::assets::{ dirs::paths::*, files::get_file };

                #current_dir

                let theme_dir = current_style();

                let style_dir = current_dir(&theme_dir);

                let style_config = get_file(style_dir.join("config.css"))
                    .contents_utf8()
                    .unwrap();

                let style_index = get_file(style_dir.join("index.css"))
                    .contents_utf8()
                    .unwrap();

                let colors = get_file(theme_dir.join("colors.css"))
                    .contents_utf8()
                    .unwrap();

                let config = get_file(theme_dir.join("config.css"))
                    .contents_utf8()
                    .unwrap();

                let index = get_file(theme_dir.join("index.css"))
                    .contents_utf8()
                    .unwrap();

                format!("{}\n{}\n{}\n{}\n{}", index, colors, config, style_config, style_index)
            };
        };
    }

    let mut icons = TokenStream::new();

    if removed_icons.is_some() {
        icons = quote! {
            type Icons = std::collections::HashMap<String, String>;

            let icons: Icons = {
                use crate::assets::{ dirs::{ get_dir, paths::* }, files::get_file };

                #current_dir

                let theme_dir = current_icons();

                let icons_dir = current_dir(&theme_dir);

                let mut map = Icons::new();

                for file in get_dir(icons_dir).files() {
                    map.insert( file.path().file_stem().unwrap().to_str().unwrap().to_owned(), file.contents_utf8().unwrap().to_string() );
                }

                map
            };
        };
    }

    let new_body = quote! {
        #style
        #icons
        #fn_body
    };

    let render_name = {
        println!("{}", emt_type.value());
        if emt_type.value() == "page" {
            return quote! { static NAME: &str = #emt_name };
        }
        TokenStream::new()
    };

    let output = quote! {
        #render_name
        use dioxus::prelude::*;
        #[component]
        pub fn #fn_name(#new_fn_args) -> Element {
            #new_body
        }
    };

    output
}
