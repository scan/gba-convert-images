use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub out_dir: String,
    pub work_dir: String,
    pub images: HashMap<String, ImageConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
            out_dir: "./src/assets".to_string(),
            work_dir: "./assets".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageConfig {
    #[serde(rename = "full")]
    FullImage(FullImageConfig),
    #[serde(rename = "tiled")]
    TiledImage(TiledImageConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseConfig {
    file_name: Option<String>,
    const_name: Option<String>,
    palette_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FullImageTargetDepth {
    #[serde(rename = "16bit")]
    U16,
    #[serde(rename = "8bit")]
    U8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullImageConfig {
    #[serde(flatten)]
    cfg: BaseConfig,

    depth: FullImageTargetDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TiledImageTargetDepth {
    #[serde(rename = "8bit")]
    U8,
    #[serde(rename = "4bit")]
    U4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiledImageConfig {
    #[serde(flatten)]
    cfg: BaseConfig,

    depth: TiledImageTargetDepth,
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_CONFIG: &str = r#"
out_dir = "./src/images"
work_dir = "./assets"

[images.image1]
type = "tiled"
depth = "8bit"

[images.image2]
type = "full"
depth = "16bit"
    "#;

    #[test]
    fn parsing() {
        let decoded: Config = toml::from_str(TEST_CONFIG).unwrap();

        assert_eq!(decoded.out_dir, "./src/images");
        assert_eq!(decoded.work_dir, "./assets");
        assert_eq!(decoded.images.len(), 2);

        match &decoded.images["image1"] {
            ImageConfig::TiledImage(cfg) => {
                assert_eq!(cfg.depth, TiledImageTargetDepth::U8);
            }
            _ => panic!(),
        };

        match &decoded.images["image2"] {
            ImageConfig::FullImage(cfg) => {
                assert_eq!(cfg.depth, FullImageTargetDepth::U16);
            }
            _ => panic!(),
        };
    }
}
