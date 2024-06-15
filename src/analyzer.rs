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
