use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::Tokenize;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi::prelude::*;

fn main() {
    let dict = make_sudachi_dict().unwrap();
    let tokenizer = StatelessTokenizer::new(&dict);
    let line: String = "狐が大好き　何を見せてくれるんだ　わぁああっ！！".to_string();

    println!("{}", line);
    let morphemes = tokenizer.tokenize(&line, Mode::C, false).unwrap();
    for morpheme in morphemes.iter() {
        println!("{:?}", morpheme);
    }
    println!("Words found: {}", morphemes.len());
}

fn make_sudachi_dict() -> Result<JapaneseDictionary, Box<dyn std::error::Error>> {
    let embedded_dictionary = decode(include_bytes!("./system_full.dic.zst"))?;
    let dictionary_file_data = SudachiDicData::new(Storage::Owned(embedded_dictionary));
    let config = Config::new_embedded()?;
    let dictionary = JapaneseDictionary::from_embedded_storage(&config, dictionary_file_data)?;
    return Ok(dictionary);
}

pub fn decode(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed)
}
