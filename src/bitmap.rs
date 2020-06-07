use quote::quote;
use proc_macro::TokenStream;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitInt, LitStr, Token};

use crate::{read_image::{ImageInfo}};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BitDepth {
    U8,
    U16,
}

impl Default for BitDepth {
    fn default() -> Self {
        return Self::U16;
    }
}

#[derive(Debug, Clone, Default)]
pub struct MacroInput {
    name: String,
    pub path: PathBuf,
    depth: BitDepth,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut img_name: Option<String> = None;
        let mut path: Option<String> = None;
        let mut depth: Option<BitDepth> = None;

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match name.to_string().as_ref() {
                "name" => {
                    if img_name.is_some() {
                        panic!("Only one `name` can be defined");
                    }

                    let img_name_str: LitStr = input.parse()?;
                    img_name = Some(img_name_str.value());
                }
                "path" => {
                    if path.is_some() {
                        panic!("Only one `path` can be defined");
                    }

                    let path_str: LitStr = input.parse()?;
                    path = Some(path_str.value());
                }
                "depth" => {
                    if depth.is_some() {
                        panic!("Only one `depth` can be defined")
                    }

                    let depth_lit: LitInt = input.parse()?;
                    let depth_val = depth_lit.value();

                    depth = match depth_val {
                        8 => Some(BitDepth::U8),
                        16 => Some(BitDepth::U16),
                        d => panic!(format!(
                            "Depth of {} is invalid, onlu 8 or 16 is supported",
                            d
                        )),
                    };
                }
                name => panic!(format!("Unknown field name: {}", name)),
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        let path = match path {
            Some(p) => Path::new(&p).to_path_buf(),
            None => panic!("path is required!"),
        };
        let name = match img_name {
            Some(n) => n,
            None => panic!("name is required!"),
        };
        let depth = depth.unwrap_or(BitDepth::U16);

        Ok(MacroInput {
            name: name,
            path: path,
            depth: depth,
        })
    }
}

impl MacroInput {
    pub fn tokens(&self, info: ImageInfo) -> TokenStream {
        let uppercase_name = self.name.to_uppercase();

        let width_name = format!("{}_WIDTH", uppercase_name);
        let height_name = format!("{}_HEIGHT", uppercase_name);
        let info_width = info.width;
        let info_height = info.height;

        let info_colours = &info.colours;
        let info_colours_length = info.colours.len();
        let info_data = &info.data;
        let info_data_length = info.data.len();

        let palette_name = format!("{}_PALETTE", uppercase_name);
        let data_name = format!("{}_BYTES", uppercase_name);

        let ast = quote! {
            pub const #width_name: usize = #info_width;
            pub const #height_name: usize = #info_height;

            pub const #palette_name: [u16; #info_colours_length] = [#(#info_colours),*];
            pub const #data_name: [u8; #info_data_length] = [#(#info_data),*];
        };


        println!("{}", ast.to_string());

        ast.into()
    }
}
