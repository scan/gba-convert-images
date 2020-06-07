use compress::rle;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, LitBool, LitInt, LitStr, Token,
};

use crate::read_image::ImageInfo;

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
    rle: bool,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut img_name: Option<String> = None;
        let mut path: Option<String> = None;
        let mut depth: Option<BitDepth> = None;
        let mut rle_enabled: Option<bool> = None;

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
                        panic!("Only one `depth` can be defined");
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
                "rle" => {
                    if rle_enabled.is_some() {
                        panic!("Only one `rle` can be defined");
                    }

                    let rle_lit: LitBool = input.parse()?;

                    rle_enabled = Some(rle_lit.value);
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
        let rle = rle_enabled.unwrap_or(false);

        Ok(MacroInput {
            name: name,
            path: path,
            depth: depth,
            rle: rle,
        })
    }
}

impl MacroInput {
    pub fn tokens(&self, info: ImageInfo) -> TokenStream {
        let uppercase_name = self.name.to_uppercase();

        let width_name = format_ident!("{}_WIDTH", uppercase_name);
        let height_name = format_ident!("{}_HEIGHT", uppercase_name);
        let info_width = info.width as usize;
        let info_height = info.height as usize;

        let dimension_ast = quote! {
            pub const #width_name: usize = #info_width;
            pub const #height_name: usize = #info_height;
        };

        let info_colours = &info.colours;
        let info_colours_length = info.colours.len();

        let palette_name = format_ident!("{}_PALETTE", uppercase_name);
        let data_name = format_ident!("{}_BYTES", uppercase_name);

        let ast = match self.depth {
            BitDepth::U8 => {
                let mut info_data = info.data;

                if self.rle {
                    let mut encoder = rle::Encoder::new(Vec::new());
                    encoder.write_all(&info_data[..]).unwrap();
                    info_data = encoder.finish().0;
                }

                let info_data_length = info_data.len();

                quote! {
                    #dimension_ast

                    pub const #palette_name: [u16; #info_colours_length] = [#(#info_colours),*];
                    pub const #data_name: [u8; #info_data_length] = [#(#info_data),*];
                }
            }
            BitDepth::U16 => {
                if self.rle {
                    panic!("RLE is not currently supported for 16 bit bitmaps")
                }

                let converted: Vec<u16> = (&info.data)
                    .into_iter()
                    .map(|b| info.colours[*b as usize])
                    .collect();

                let converted_length = converted.len();

                quote! {
                    #dimension_ast

                    pub const #data_name: [u16; #converted_length] = [#(#converted),*];
                }
            }
        };

        ast.into()
    }
}
