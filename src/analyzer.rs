use std::collections::HashMap;

pub fn analyze_morphemes(morpheme_surfaces: &Vec<String>) {
    let counts: HashMap<String, i32> = morpheme_surfaces.into_iter().fold(HashMap::new(), |mut map, c| {*map.entry(c.to_string()).or_insert(0) += 1; map});

    println!("{:?}", counts);
}
