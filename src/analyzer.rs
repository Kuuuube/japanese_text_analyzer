use std::collections::HashMap;

pub fn generate_occurrence_list(morpheme_surfaces: &Vec<String>) -> HashMap<String, i32> {
    return morpheme_surfaces.into_iter().fold(HashMap::new(), |mut map, c| {*map.entry(c.to_string()).or_insert(0) += 1; map});
}

pub fn sort_occurrence_list(occurrence_list: &HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut occurrence_list_sorted: Vec<(String, i32)> = occurrence_list.iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();
    occurrence_list_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    return occurrence_list_sorted;
}

pub fn find_single_occurrences(occurrence_list: &HashMap<String, i32>) -> Vec<String> {
    return occurrence_list.iter().fold(Vec::new(), |mut map: Vec<String>, x| {if *x.1 == 1 {map.push(x.0.to_owned())}; map});
}

pub fn filter_non_kanji(string: &String) -> Vec<char> {
    return string.chars().filter(|x| check_if_kanji(*x as u32)).collect();
}

fn check_if_kanji(codepoint: u32) -> bool {
    //CJK Unified Ideographs
    if codepoint >= 0x4E00 && codepoint <= 0x9FFF ||
    //CJK Unified Ideographs Extension A
    codepoint >= 0x3400 && codepoint <= 0x4DBF ||
    //CJK Unified Ideographs Extension B
    codepoint >= 0x20000 && codepoint <= 0x2A6DF ||
    //CJK Unified Ideographs Extension C
    codepoint >= 0x2A700 && codepoint <= 0x2B73F ||
    //CJK Unified Ideographs Extension D
    codepoint >= 0x2B740 && codepoint <= 0x2B81F ||
    //CJK Unified Ideographs Extension E
    codepoint >= 0x2B820 && codepoint <= 0x2CEAF ||
    //CJK Unified Ideographs Extension F
    codepoint >= 0x2CEB0 && codepoint <= 0x2EBEF ||
    //CJK Unified Ideographs Extension G
    codepoint >= 0x30000 && codepoint <= 0x3134F ||
    //CJK Unified Ideographs Extension H
    codepoint >= 0x31350 && codepoint <= 0x323AF ||
    //CJK Compatibility Ideographs
    codepoint >= 0xF900 && codepoint <= 0xFAFF {
        return true;
    }
    return false;
}
