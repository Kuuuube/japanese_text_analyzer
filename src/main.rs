use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::Tokenize;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi::prelude::*;

fn main() {
    let dict = make_sudachi_dict();
    let tokenizer = StatelessTokenizer::new(&dict);
    let line: String = "狐が大好き　何を見せてくれるんだ　わぁああっ！！".to_string();

    println!("{}", line);
    let morphemes = tokenizer.tokenize(&line, Mode::C, false).unwrap();
    for morpheme in morphemes.iter() {
        println!("{:?}", morpheme);
    }
    println!("Words found: {}", morphemes.len());
}

fn make_sudachi_dict() -> JapaneseDictionary {
    let embedded_dictionary = include_bytes!("./system_full.dic");
    let dictionary_file_data = SudachiDicData::new(Storage::Borrowed(embedded_dictionary));
    let config = Config::new_embedded().unwrap();
    let dictionary = JapaneseDictionary::from_embedded_storage(&config, dictionary_file_data).unwrap();
    return dictionary;
}
