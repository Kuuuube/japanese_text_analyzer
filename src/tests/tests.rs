#[cfg(test)]
#[test]
pub fn parse_minimal_synthetic_json() {
    let json_files = crate::json_handler::get_json_files("./src/tests/data/minimal_synthetic.json").unwrap();
    println!("{:?}", json_files);

    let lines = crate::json_handler::get_json_file_data(json_files);
    let expected_lines = vec![
        "医薬品安全管理責任者".to_owned(),
        "消費者安全調査委員会".to_owned(),
        "さっぽろテレビ塔".to_owned(),
        "カンヌ国際映画祭".to_owned(),
    ];
    assert!(lines == expected_lines)
}
