#[cfg(test)]
#[test]
pub fn parse_minimal_synthetic_json() {
    //load file and extract text
    let json_files = crate::file_handler::get_files("./src/tests/data/minimal_synthetic.json", ".json");
    println!("{:?}", json_files);

    let lines = crate::file_handler::get_json_file_data(json_files);
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(lines == expected_lines);

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenized_data = crate::run_tokenization(&lines, &dict);
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
pub fn parse_minimal_synthetic_txt() {
    //load file and extract text
    let txt_files = crate::file_handler::get_files("./src/tests/data/minimal_synthetic.txt", ".txt");
    println!("{:?}", txt_files);

    let lines = crate::file_handler::get_plain_file_data(txt_files);
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(lines == expected_lines);

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenized_data = crate::run_tokenization(&lines, &dict);
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
    println!("{:?}", any_files);

    let lines = crate::file_handler::get_plain_file_data(any_files);
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(lines == expected_lines);

    //tokenize text
    let dict: sudachi::dic::dictionary::JapaneseDictionary =
        crate::dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    let tokenized_data = crate::run_tokenization(&lines, &dict);
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
