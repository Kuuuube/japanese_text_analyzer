use std::path::PathBuf;

use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::Tokenize;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::prelude::*;

fn main() {
    let config = Config::new(None, None, Some(PathBuf::from("./system_full.dic",))).unwrap(); //http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/
    let dict = JapaneseDictionary::from_cfg(&config).unwrap();
    let tokenizer = StatelessTokenizer::new(&dict);
    let line: String = "狐が大好き　何を見せてくれるんだ　わぁああっ！！".to_string();

    println!("{}", line);
    let morphemes = tokenizer.tokenize(&line, Mode::C, false).unwrap();
    for morpheme in morphemes.iter() {
        println!("{:?}", morpheme);
    }
    println!("Words found: {}", morphemes.len());
}
