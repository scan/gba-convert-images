use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::path::{Path, PathBuf};
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, LitInt, LitStr, Token,
};

use crate::{
    read_image::ImageInfo,
    util::{consolidate_u16_u32, consolidate_u4_u32, consolidate_u8_u32},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BitDepth {
    U4,
    U8,
}

impl Default for BitDepth {
    fn default() -> Self {
        return Self::U8;
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
                        panic!("Only one `depth` can be defined");
                    }

                    let depth_lit: LitInt = input.parse()?;
                    let depth_val = depth_lit.value();

                    depth = match depth_val {
                        4 => Some(BitDepth::U4),
                        8 => Some(BitDepth::U8),
                        d => panic!(format!(
                            "Depth of {} is invalid, only 4 or 8 is supported for tilesets",
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
        let depth = depth.unwrap_or(BitDepth::default());

        Ok(MacroInput {
            name: name,
            path: path,
            depth: depth,
        })
    }
}

impl MacroInput {
    pub fn tokens(&self, info: ImageInfo) -> TokenStream {
        if info.width % 8 != 0 {
            panic!(format!(
                "image width of {} is not evenly divisible by 8",
                info.width
            ));
        }

        if info.height % 8 != 0 {
            panic!(format!(
                "image width of {} is not evenly divisible by 8",
                info.height
            ));
        }

        let cols = (info.width / 8) as usize;
        let rows = (info.height / 8) as usize;
        let num_tiles = cols * rows;

        let mut tiles: Vec<Vec<u8>> = vec![];

        for row in 0..rows {
            for col in 0..cols {
                let mut tile: Vec<u8> = vec![];

                for y in 0..8 {
                    for x in 0..8 {
                        tile.push(info.get(col + x, row + y));
                    }
                }

                tiles.push(tile);
            }
        }

        let tiles: Vec<u8> = tiles.into_iter().flatten().collect();

        let uppercase_name = self.name.to_uppercase();

        let tiles_count_name = format_ident!("{}_TILES_COUNT", uppercase_name);

        let palette_name = format_ident!("{}_PALETTE", uppercase_name);
        let info_colours = consolidate_u16_u32(info.colours);
        let info_colours_length = info_colours.len();

        let dimension_ast = quote! {
            pub const #tiles_count_name: usize = #num_tiles;
            pub const #palette_name: [u32, #info_colours_length] = [#(#info_colours),*];
        };

        let tiles = match self.depth {
            BitDepth::U8 => consolidate_u8_u32(tiles),
            BitDepth::U4 => {
                if info_colours.len() > 16 {
                    panic!("Too many colours in palette for 16 bit tilemap!");
                }
                consolidate_u4_u32(tiles)
            }
        };

        let tiles_length = tiles.len();
        let tiles_name = format_ident!("{}_TILES", uppercase_name);

        let ast = quote! {
            #dimension_ast

            pub const #tiles_name: [u32; #tiles_length] = [#(#tiles),*];
        };

        return ast.into();
    }
}
