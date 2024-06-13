use std::path::PathBuf;

use sudachi::analysis::stateful_tokenizer::StatefulTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::subset::InfoSubset;
use sudachi::prelude::*;

fn main() {
    let config = Config::new(None, None, Some(PathBuf::from("./system_full.dic",))).unwrap(); //http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/
    let dict = JapaneseDictionary::from_cfg(&config).unwrap();
    let mut tokenizer = StatefulTokenizer::new(&dict, Mode::C);
    tokenizer.set_subset(InfoSubset::empty());
    let mut morphemes = MorphemeList::empty(&dict);
    let line: String = "狐が大好き　何を見せてくれるんだ　わぁああっ！！".to_string();

    println!("{}", line);
    tokenizer.reset().push_str(&line);
    tokenizer.do_tokenize().unwrap();
    morphemes.collect_results(&mut tokenizer).unwrap();
    for morpheme in morphemes.iter() {
        println!("{:?}", morpheme);
    }
    println!("Words found: {}", morphemes.len());
}
