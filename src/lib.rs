mod bitmap;
mod read_image;

use proc_macro::TokenStream;
use std::{env, path::Path};
use syn::parse_macro_input;

use crate::{bitmap::MacroInput, read_image::read_image};

#[proc_macro]
pub fn bitmap(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let full_path = Path::new(&root).join(&input.path);

    let info = if full_path.is_file() {
        read_image(full_path)
    } else {
        panic!(format!(
            "path `{}` is not a valid file",
            full_path.to_string_lossy()
        ));
    };

    input.tokens(info)
}
