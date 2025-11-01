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

pub fn get_json_file_data(filepath: PathBuf) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Default::default();
    let json_data = match std::fs::read_to_string(&filepath) {
        Ok(ok) => ok,
        Err(err) => {
            let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
            println!(
                "Failed to read json file `{}`\nError: `{}`",
                filepath_str, err
            );
            return None;
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
    return Some(lines);
}

pub fn get_mokuro_file_data(filepath: PathBuf) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Default::default();
    let json_data = match std::fs::read_to_string(&filepath) {
        Ok(ok) => ok,
        Err(err) => {
            let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
            println!(
                "Failed to read json file `{}`\nError: `{}`",
                filepath_str, err
            );
            return None;
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
    return Some(lines);
}

pub fn get_plain_file_data(filepath: PathBuf) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Default::default();
    let txt_strings: Vec<String> = match std::fs::read_to_string(&filepath) {
        Ok(ok) => ok.split("\n").map(|x| x.to_owned()).collect(),
        Err(err) => {
            let filepath_str = filepath.to_str().unwrap_or("failed to display filepath");
            println!("Failed to read file `{}`\nError: `{}`", filepath_str, err);
            return None;
        }
    };
    for txt_string in txt_strings {
        let filtered_txt_strings = crate::analyzer::filter_duplicate_ascii(txt_string);
        for filtered_txt_string in filtered_txt_strings {
            if filtered_txt_string.len() > SUDACHI_MAX_TOKENIZER_LENGTH {
                lines.append(&mut chunk_utf8_string(
                    filtered_txt_string,
                    SUDACHI_MAX_TOKENIZER_LENGTH,
                ));
            } else {
                lines.push(filtered_txt_string);
            }
        }
    }
    return Some(lines);
}

fn chunk_utf8_string(input_string: String, chunk_size: usize) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];
    let mut current_chunk: String = "".to_string();
    for char in input_string.chars() {
        if current_chunk.len() + char.len_utf8() < chunk_size {
            current_chunk += &char.to_string();
        } else {
            chunks.push(current_chunk);
            current_chunk = char.to_string();
        }
    }
    chunks.push(current_chunk);
    return chunks;
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
