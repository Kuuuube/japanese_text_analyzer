mod json_handler;
mod dict_handler;
mod analyzer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start_directory_path = args.get(1).unwrap();
    println!("Finding json files in {}", start_directory_path);

    let json_files = json_handler::get_json_files(start_directory_path).unwrap();
    println!("Found {} json files", json_files.len());

    let lines = json_handler::get_json_file_data(json_files);
    println!("Extracted {} lines of text", lines.len());

    println!("Loading tokenizer dictionary");
    let dict = dict_handler::make_sudachi_dict().unwrap();
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);

    let mut morpheme_surfaces: Vec<String> = Default::default();

    println!("Running tokenizer");
    for line in lines {
        let morphemes = sudachi::analysis::Tokenize::tokenize(&tokenizer, &line, sudachi::analysis::Mode::C, false).unwrap();
        for morpheme in morphemes.iter() {
            morpheme_surfaces.push(morpheme.surface().to_string());
        }
    }

    let occurrence_list = analyzer::generate_occurrence_list(&morpheme_surfaces);
    let occurrence_list_sorted = analyzer::sort_occurrence_list(&occurrence_list);
    let single_occurrences = analyzer::find_single_occurrences(&occurrence_list);
    let characters = morpheme_surfaces.join("");

    let stats: AnalysisStats = AnalysisStats {
        char_count: characters.len(),
        kanji_count: 0,
        unique_kanji_count: 0,
        word_count: morpheme_surfaces.len(),
        unique_word_count: occurrence_list_sorted.len(),
        unique_word_count_single_occurrence: single_occurrences.len(),
    };

    println!("{:?}", stats);
}

#[derive(Debug, Default)]
struct AnalysisStats {
    char_count: usize,
    kanji_count: usize,
    unique_kanji_count: usize,
    word_count: usize,
    unique_word_count: usize,
    unique_word_count_single_occurrence: usize,
}
