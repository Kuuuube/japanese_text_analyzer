use serde::Deserialize;
use std::path::PathBuf;

//https://github.com/WorksApplications/sudachi.rs/blob/d78bf49e8473a5895e542c54f9e7375e9c009e26/sudachi/src/input_text/buffer/mod.rs#L32C27-L32C52
const SUDACHI_MAX_TOKENIZER_LENGTH: usize = u16::MAX as usize / 4 * 3;

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

pub fn get_mokuro_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
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
        match serde_json::from_str::<MokuroFile>(&json_data) {
            Ok(ok) => {
                for page in ok.pages {
                    for block in page.blocks {
                        lines.push(block.lines.concat());
                    }
                }
            }
            Err(_) => {}
        }
    }
    return lines;
}

pub fn get_plain_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
    let mut lines: Vec<String> = Default::default();
    for filepath in filepaths {
        let txt_data: Vec<String> = match std::fs::read_to_string(&filepath) {
            Ok(ok) => ok.split("\n").map(|x| x.to_owned()).collect(),
            Err(err) => {
                let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
                println!(
                    "Failed to read file `{}`\nError: `{}`",
                    filepath_str, err
                );
                continue;
            }
        };
        for data in txt_data {
            if data.len() > SUDACHI_MAX_TOKENIZER_LENGTH {
                let mut current_string = data.clone();
                while current_string.len() > 0 {
                    let split_string: String = if current_string.len() > SUDACHI_MAX_TOKENIZER_LENGTH {
                        let current_string_clone = current_string.clone();
                        let split_string = current_string_clone.as_bytes().split_at(SUDACHI_MAX_TOKENIZER_LENGTH);
                        current_string = String::from_utf8_lossy(split_string.1).to_string();
                        let expected_string_len = split_string.0.len();
                        let lossy_string = String::from_utf8_lossy(split_string.0).to_string();
                        if lossy_string.len() > expected_string_len { //if `from_utf8_lossy` creates a replacement character `�` it needs to be chopped off
                            let mut lossy_chars = lossy_string.chars();
                            lossy_chars.next_back();
                            lossy_chars.collect()
                        } else {
                            lossy_string
                        }
                    } else {
                        current_string = "".to_string();
                        current_string.clone()
                    };
                    lines.push(split_string);
                }
            } else {
                lines.push(data);
            }
        }
    }
    return lines;
}

#[derive(Debug, Deserialize)]
struct MokuroFile {
    //version: String,
    //title: String,
    //title_uuid: String,
    //volume: String,
    //volume_uuid: String,
    pages: Vec<MokuroJson>,
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
