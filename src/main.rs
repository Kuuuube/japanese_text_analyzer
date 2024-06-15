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

    println!("Analyzing results");
    let characters = morpheme_surfaces.join("");

    let word_occurrence_list = analyzer::generate_occurrence_list(&morpheme_surfaces);
    let characters_occurrence_list = analyzer::generate_occurrence_list(&characters.chars().collect());

    let word_occurrence_list_sorted = analyzer::sort_occurrence_list(&word_occurrence_list);
    let word_count_single_occurrence = analyzer::find_single_occurrences(&word_occurrence_list);

    let characters_count_single_occurrence = analyzer::find_single_occurrences(&characters_occurrence_list);
    let kanji_count_single_occurrence = analyzer::filter_non_kanji(&characters_count_single_occurrence);

    let japanese_characters = analyzer::filter_non_japanese(&characters.chars().collect());
    let kanji_characters = analyzer::filter_non_kanji(&characters.chars().collect());
    let mut unique_kanji_characters: Vec<char> = kanji_characters.clone();
    unique_kanji_characters.sort();
    unique_kanji_characters.dedup();

    let stats: AnalysisStats = AnalysisStats {
        char_count: japanese_characters.len(),
        kanji_count: kanji_characters.len(),
        unique_kanji_count: unique_kanji_characters.len(),
        kanji_count_single_occurrence: kanji_count_single_occurrence.len(),
        word_count: morpheme_surfaces.len(),
        unique_word_count: word_occurrence_list_sorted.len(),
        word_count_single_occurrence: word_count_single_occurrence.len(),
    };

    println!("{}{}\n{}{}\n{}{}\n{}{} ({} of unique kanji)\n{}{}\n{}{} ({} of all words)\n{}{} ({} of unique words)\n{}{}",
        "Number of Japanese characters: ", stats.char_count,
        "Number of Kanji characters: ", stats.kanji_count,
        "Number of unique kanji: ", stats.unique_kanji_count,
        "Number of unique kanji appearing only once: ", stats.kanji_count_single_occurrence, analyzer::get_fancy_percentage(stats.unique_kanji_count, stats.kanji_count_single_occurrence),
        "Number of words in total: ", stats.word_count,
        "Number of unique words: ", stats.unique_word_count, analyzer::get_fancy_percentage(stats.word_count, stats.unique_word_count),
        "Number of words appearing only once: ", stats.word_count_single_occurrence, analyzer::get_fancy_percentage(stats.unique_word_count, stats.word_count_single_occurrence),
        "Average length of a sentence: ", ""
    );
}

#[derive(Debug, Default)]
struct AnalysisStats {
    char_count: usize,
    kanji_count: usize,
    unique_kanji_count: usize,
    kanji_count_single_occurrence: usize,
    word_count: usize,
    unique_word_count: usize,
    word_count_single_occurrence: usize,
}
