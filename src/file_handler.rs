use serde::Deserialize;
use std::path::PathBuf;

pub fn get_files(directory: &str, extension: &str) -> Vec<std::path::PathBuf> {
    let mut json_files: Vec<std::path::PathBuf> = Default::default();
    for entry in walkdir::WalkDir::new(directory)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();

        if file_name.to_string().ends_with(extension) {
            json_files.push(entry.into_path());
        }
    }
    return json_files;
}

pub fn get_json_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
    let mut lines: Vec<String> = Default::default();
    for filepath in filepaths {
        let json_data = match std::fs::read_to_string(&filepath) {
            Ok(ok) => ok,
            Err(err) => {
                let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
                println!(
                    "Failed to read json file `{}`\nError: `{}`",
                    filepath_str, err
                );
                continue;
            }
        };
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

pub fn get_txt_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
    let mut lines: Vec<String> = Default::default();
    for filepath in filepaths {
        let txt_data: Vec<String> = match std::fs::read_to_string(&filepath) {
            Ok(ok) => ok.split("\n").map(|x| x.to_owned()).collect(),
            Err(err) => {
                let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
                println!(
                    "Failed to read txt file `{}`\nError: `{}`",
                    filepath_str, err
                );
                continue;
            }
        };
        for data in txt_data {
            lines.push(data);
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
