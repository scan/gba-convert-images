use std::fs::{ write };
use crate::{ config::{ Config, ImageConfig }, read_image::{ read_image, ImageInfo } };
use std::path::{Path, PathBuf};

const SUPPORTED_FILE_ENDINGS: &'static [&'static str] = &["png", "tga", "jpg", "jpeg"];

fn file_name_for_key(base_path: &Path, key: &str) -> Option<PathBuf> {
    let mut path = base_path.clone().join(key);

    for extenstion in SUPPORTED_FILE_ENDINGS {
        path.set_extension(extenstion);

        if path.exists() {
            return Some(path);
        }
    }

    return None;
}

fn image_source(info: ImageInfo, config: ImageConfig) -> String {

}

pub fn convert_images(config: Config) {
    let source_dir = Path::new(&config.work_dir);
    let target_dir = Path::new(&config.out_dir);

    for (key, image_config) in config.images {
        // TODO: Check for error
        let source_file = file_name_for_key(source_dir, &key).unwrap();
        let image_info = read_image(source_file);
        let img_source = image_source(image_info, image_config);

        let mut target_file = target_dir.join(&key);
        target_file.set_extension("rs");

        write(target_file, img_source).expect("Unable to write file");
    }
}
