use std::path::PathBuf;

use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::Tokenize;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi::prelude::*;
use serde::Deserialize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start_directory_path = args.get(1).unwrap();
    println!("Finding json files in {}", start_directory_path);

    let json_files = get_json_files(start_directory_path).unwrap();
    println!("Found {} json files", json_files.len());

    let lines = get_json_file_data(json_files);
    println!("Extracted {} lines of text", lines.len());

    println!("Loading tokenizer dictionary");
    let dict = make_sudachi_dict().unwrap();
    let tokenizer = StatelessTokenizer::new(&dict);

    let mut morpheme_surfaces: Vec<String> = Default::default();

    for line in lines {
        let morphemes = tokenizer.tokenize(&line, Mode::C, false).unwrap();
        for morpheme in morphemes.iter() {
            morpheme_surfaces.push(morpheme.surface().to_string());
        }
    }
    println!("{}", morpheme_surfaces.join("|"));
    println!("Morphemes found: {}", morpheme_surfaces.len());
}

fn make_sudachi_dict() -> Result<JapaneseDictionary, Box<dyn std::error::Error>> {
    let embedded_dictionary = decode_zstd(include_bytes!("./system_full.dic.zst"))?;
    let dictionary_file_data = SudachiDicData::new(Storage::Owned(embedded_dictionary));
    let config = Config::new_embedded()?;
    let dictionary = JapaneseDictionary::from_embedded_storage(&config, dictionary_file_data)?;
    return Ok(dictionary);
}

fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed)
}

fn get_json_files(directory: &str) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let mut json_files: Vec<std::path::PathBuf> = Default::default();
    for entry in walkdir::WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();

        if file_name.to_string().ends_with(".json") {
            json_files.push(entry.into_path());
        }
    }
    return Ok(json_files);
}

fn get_json_file_data(filepaths: Vec<PathBuf>) -> Vec<String> {
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
    blocks: Vec<MokuroBlock>
}

#[derive(Debug, Deserialize)]
struct MokuroBlock {
    //#[serde(rename = "box")]
    //ocr_box: Vec<i32>,
    //vertical: bool,
    //font_size: i32,
    //lines_coords: Vec<Vec<Vec<f32>>>,
    lines: Vec<String>
}
