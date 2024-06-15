use std::collections::HashMap;

pub fn generate_occurrence_list(morpheme_surfaces: &Vec<String>) -> Vec<(String, i32)> {
    let morpheme_counts: HashMap<String, i32> = morpheme_surfaces.into_iter().fold(HashMap::new(), |mut map, c| {*map.entry(c.to_string()).or_insert(0) += 1; map});
    let mut morpheme_counts_sorted: Vec<(String, i32)> = morpheme_counts.iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();
    morpheme_counts_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    return morpheme_counts_sorted;
}
