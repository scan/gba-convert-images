use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    out_dir: String,
    work_dir: String,
    images: Vec<ImageConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            images: vec![],
            out_dir: "./src/assets".to_string(),
            work_dir: "./assets".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageConfig {
    FullImage(FullImageConfig),
    TiledImage(TiledImageConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseConfig {
    file_name: String,
    const_name: Option<String>,
    palette_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum FullImageTargetDepth {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TiledImageTargetDepth {
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