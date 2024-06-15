use std::collections::HashMap;

pub fn generate_occurrence_list(morpheme_surfaces: &Vec<String>) -> HashMap<String, i32> {
    return morpheme_surfaces.into_iter().fold(HashMap::new(), |mut map, c| {*map.entry(c.to_string()).or_insert(0) += 1; map});
}

pub fn sort_occurrence_list(occurrence_list: &HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut occurrence_list_sorted: Vec<(String, i32)> = occurrence_list.iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();
    occurrence_list_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    return occurrence_list_sorted;
}

pub fn find_single_occurrences<T: ToOwned<Owned = T>>(occurrence_list: &HashMap<T, i32>) -> Vec<T> {
    return occurrence_list.iter().fold(Vec::new(), |mut map: Vec<T>, x: (&T, &i32)| {if *x.1 == 1 {map.push(x.0.to_owned())}; map});
}

pub fn filter_non_japanese(string: &String) -> Vec<char> {
    return string.chars().filter(|x| check_if_japanese(*x as u32)).collect();
}

pub fn filter_non_kanji(string: &String) -> Vec<char> {
    return string.chars().filter(|x| check_if_kanji(*x as u32)).collect();
}

fn check_if_japanese(codepoint: u32) -> bool {
    //Kanji
    if check_if_kanji(codepoint) ||
    //Hiragana (punctuation excluded: U+3099..U+309E; full range: U+3040..U+309F)
    codepoint >= 0x3040 && codepoint <= 0x3096 || codepoint == 0x309F ||
    //Katakana (punctuation excluded U+30A0, U+30FB..U+30FF; full range: U+30A0..U+30FF)
    codepoint >= 0x30A1 && codepoint <= 0x30FA ||
    //Half-width Katakana (non-japanese excluded: U+FF01..U+FF63, U+FFA0..U+FFEF; japanese sound marks excluded: U+FF9E..U+FF9F; japanese punctuation excluded: U+FF64..U+FF65; full range: U+FF00..U+FFEF)
    codepoint >= 0xFF66 && codepoint <= 0xFF9D ||
    //Small Kana Extension
    codepoint >= 0x1B130 && codepoint <= 0x1B16F ||
    //Kana Extended A (Hentaigana and reserved small kana punctuation) (Reserved punctuation excluded: U+1B12B..U+1B12F; full range: U+1B100..U+1B12F)
    codepoint >= 0x1B100 && codepoint <= 0x1B122 ||
    //Kana Supplement (Hentaigana)
    codepoint >= 0x1B000 && codepoint <= 0x1B0FF {
        return true;
    }
    return false;
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
