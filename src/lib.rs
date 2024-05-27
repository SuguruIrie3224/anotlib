use serde::{Deserialize, Serialize};

// data structures for the JSON file
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub filename: String,
    pub size: ImageSize,
    pub base64: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub label: String,
    pub coordinates: Coordinates,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub image: Image,
    pub bounding_boxes: Vec<BoundingBox>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    pub annotations: Vec<Annotation>,
}


// read/write JSON file

use std::fs::File;
use std::io::{self, Read, Write};

impl Annotations {
    // .anotファイルを読み込む関数
    pub fn from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let annotations: Annotations = serde_json::from_str(&contents)?;
        Ok(annotations)
    }

    // .anotファイルに書き込む関数
    pub fn to_file(&self, path: &str) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    // 新しいアノテーションを追加する関数
    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }

    // すべてのアノテーションを返す関数
    pub fn get_annotations(&self) -> &Vec<Annotation> {
        &self.annotations
    }
}


