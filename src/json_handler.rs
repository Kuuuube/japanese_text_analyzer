use serde::Deserialize;
use std::path::PathBuf;

pub fn get_json_files(directory: &str) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let mut json_files: Vec<std::path::PathBuf> = Default::default();
    for entry in walkdir::WalkDir::new(directory)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();

        if file_name.to_string().ends_with(".json") {
            json_files.push(entry.into_path());
        }
    }
    return Ok(json_files);
}

pub fn get_json_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
    let mut lines: Vec<String> = Default::default();
    for filepath in filepaths {
        let json_data = std::fs::read_to_string(filepath).unwrap();
        match serde_json::from_str::<MokuroJson>(&json_data) {
            Ok(ok) => {
                for block in ok.blocks {
                    lines.push(block.lines.concat());
                }
            }
            Err(_) => {}
        }
    }
    return lines;
}

#[derive(Debug, Deserialize)]
struct MokuroJson {
    //version: String,
    //img_width: i32,
    //img_height: i32,
    blocks: Vec<MokuroBlock>,
}

#[derive(Debug, Deserialize)]
struct MokuroBlock {
    //#[serde(rename = "box")]
    //ocr_box: Vec<i32>,
    //vertical: bool,
    //font_size: i32,
    //lines_coords: Vec<Vec<Vec<f32>>>,
    lines: Vec<String>,
}
