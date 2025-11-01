#[cfg(test)]
#[test]
pub fn parse_minimal_synthetic_json() {
    //load file and extract text
    let json_files =
        crate::file_handler::get_files("./src/tests/data/minimal_synthetic.json", ".json");
    dbg!("{:?}", &json_files);
    assert!(json_files.len() == 1);

    let maybe_lines =
        crate::file_handler::get_json_file_data(json_files.get(0).unwrap().to_path_buf());
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(maybe_lines == Some(expected_lines));

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);
    let mut tokenized_data = vec![];
    if let Some(lines) = maybe_lines {
        tokenized_data = crate::run_tokenization(&lines, &tokenizer);
    }
    let expected_tokenized_data = vec![
        "医薬品".to_owned(),
        "安全".to_owned(),
        "管理".to_owned(),
        "責任者".to_owned(),
        "消費者".to_owned(),
        "安全".to_owned(),
        "調査".to_owned(),
        "委員会".to_owned(),
        "さっぽろ".to_owned(),
        "テレビ塔".to_owned(),
        "カンヌ".to_owned(),
        "国際".to_owned(),
        "映画祭".to_owned(),
    ];
    assert!(tokenized_data == expected_tokenized_data);
}

#[test]
pub fn parse_minimal_synthetic_any() {
    //load file and extract text
    let any_files = crate::file_handler::get_files("./src/tests/data/minimal_synthetic.txt", "");
    dbg!("{:?}", &any_files);
    assert!(any_files.len() == 1);

    let maybe_lines =
        crate::file_handler::get_plain_file_data(any_files.get(0).unwrap().to_path_buf());
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(maybe_lines == Some(expected_lines));

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);
    let mut tokenized_data = vec![];
    if let Some(lines) = maybe_lines {
        tokenized_data = crate::run_tokenization(&lines, &tokenizer);
    }
    let expected_tokenized_data = vec![
        "医薬品".to_owned(),
        "安全".to_owned(),
        "管理".to_owned(),
        "責任者".to_owned(),
        "消費者".to_owned(),
        "安全".to_owned(),
        "調査".to_owned(),
        "委員会".to_owned(),
        "さっぽろ".to_owned(),
        "テレビ塔".to_owned(),
        "カンヌ".to_owned(),
        "国際".to_owned(),
        "映画祭".to_owned(),
    ];
    assert!(tokenized_data == expected_tokenized_data);
}
