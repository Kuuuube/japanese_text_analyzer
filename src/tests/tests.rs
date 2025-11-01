#[cfg(test)]
const EXPECTED_LINES: [[&str; 4]; 1] = [[
    "医薬品安全管理責任者",
    "消費者安全調査委員会",
    "さっぽろテレビ塔",
    "カンヌ国際映画祭",
]];
#[cfg(test)]
const EXPECTED_TOKENIZED_DATA: [&str; 13] = [
    "医薬品",
    "安全",
    "管理",
    "責任者",
    "消費者",
    "安全",
    "調査",
    "委員会",
    "さっぽろ",
    "テレビ塔",
    "カンヌ",
    "国際",
    "映画祭",
];

#[cfg(test)]
#[test]
pub fn parse_minimal_synthetic_json() {
    //load file and extract text
    let json_files =
        crate::file_handler::get_files("./src/tests/data/minimal_synthetic.json", ".json");
    dbg!("{:?}", &json_files);
    assert!(json_files.len() == 1);

    let lines =
        crate::file_handler::get_json_file_data(json_files.get(0).unwrap().to_path_buf());
    assert!(vec![lines.clone()] == EXPECTED_LINES);

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);
    let mut tokenized_data = vec![];
    tokenized_data.append(&mut crate::run_tokenization(&lines, &tokenizer));
    assert!(tokenized_data == EXPECTED_TOKENIZED_DATA);
}

#[test]
pub fn parse_minimal_synthetic_any() {
    //load file and extract text
    let any_files = crate::file_handler::get_files("./src/tests/data/minimal_synthetic.txt", "");
    dbg!("{:?}", &any_files);
    assert!(any_files.len() == 1);

    let lines_groupings =
        crate::file_handler::BufferedPlainLineReader::new(&any_files.get(0).unwrap().to_path_buf()).unwrap();
    let mut all_lines = vec![];
    for lines in lines_groupings {
        all_lines.push(lines);
    }
    assert!(all_lines == EXPECTED_LINES);

    //tokenize text
    let lines_groupings =
        crate::file_handler::BufferedPlainLineReader::new(&any_files.get(0).unwrap().to_path_buf()).unwrap();
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);
    let mut tokenized_data = vec![];
    for lines in lines_groupings {
        tokenized_data.append(&mut crate::run_tokenization(&lines, &tokenizer));
    }
    dbg!(&tokenized_data);
    assert!(tokenized_data == EXPECTED_TOKENIZED_DATA);
}
