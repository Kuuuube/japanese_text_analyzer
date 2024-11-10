use args_parser::AnalysisType;
use sudachi::dic::dictionary::JapaneseDictionary;

mod analyzer;
mod args_parser;
mod dict_handler;
mod file_handler;
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = args_parser::get_args(args);

    let (media_type, enumeration_name) = match parsed_args.analysis_type {
        AnalysisType::MokuroJson => ("manga volumes", "pages"),
        AnalysisType::Mokuro => ("path", "manga volumes"),
        AnalysisType::Any => ("path", "files"),
    };

    println!("Finding {} in {}", media_type, parsed_args.start_path);
    let start_time = std::time::Instant::now();
    let files = file_handler::get_files(&parsed_args.start_path, &parsed_args.extension);
    let file_count = files.len();
    let dir_count = analyzer::count_directories(&files);
    println!(
        "Found {} {} from {} {} ({}ms)",
        file_count,
        enumeration_name,
        dir_count,
        media_type,
        start_time.elapsed().as_millis()
    );

    println!("Extracting text from {}", enumeration_name);
    let start_time = std::time::Instant::now();
    let lines = match parsed_args.analysis_type {
        AnalysisType::MokuroJson => file_handler::get_json_file_data(files),
        AnalysisType::Mokuro => file_handler::get_mokuro_file_data(files),
        AnalysisType::Any => file_handler::get_plain_file_data(files),
    };
    println!(
        "Extracted {} lines of text ({}ms)",
        lines.len(),
        start_time.elapsed().as_millis()
    );

    println!("Loading tokenizer dictionary");
    let start_time = std::time::Instant::now();
    let dict = dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    println!("Dictionary loaded ({}ms)", start_time.elapsed().as_millis());

    println!("Running tokenizer");
    let start_time = std::time::Instant::now();
    let morpheme_surfaces = run_tokenization(&lines, &dict);
    println!(
        "Tokenizer finished ({}ms)",
        start_time.elapsed().as_millis()
    );

    println!("Analyzing results");
    let start_time = std::time::Instant::now();
    let stats = get_stats(lines, morpheme_surfaces, file_count, dir_count);
    println!(
        "Analysis completed ({}ms)",
        start_time.elapsed().as_millis()
    );

    let format_specific_stats = match parsed_args.analysis_type {
        AnalysisType::MokuroJson => format!("{}{} ({} total volumes)\n{}{} ({} total pages)\n{}{} (shortest: {}) (longest: {}) ({} total textboxes)",
            "Average volume length in characters: ", stats.avg_volume_length, stats.volume_count,
            "Average page length in characters: ", stats.avg_page_length, stats.page_count,
            "Average textbox length in characters: ", stats.avg_box_length, stats.shortest_box_length, stats.longest_box_length, stats.box_count),
        AnalysisType::Any => "".to_string(),
        AnalysisType::Mokuro => format!("{}{} (shortest: {}) (longest: {}) ({} total textboxes)",
        "Average textbox length in characters: ", stats.avg_box_length, stats.shortest_box_length, stats.longest_box_length, stats.box_count),
    };

    let formatted_stats = format!("{}\n{}\n{}{}\n{}{}\n{}{}\n{}{} ({} of unique kanji)\n{}{}\n{}{} ({} of all words)\n{}{} ({} of unique words)\n{}",
        parsed_args.start_path,
        "----------------------------------------------------------------------------",
        "Number of Japanese characters: ", stats.char_count,
        "Number of kanji characters: ", stats.kanji_count,
        "Number of unique kanji: ", stats.unique_kanji_count,
        "Number of unique kanji appearing only once: ", stats.kanji_count_single_occurrence, analyzer::get_fancy_percentage(stats.unique_kanji_count, stats.kanji_count_single_occurrence),
        "Number of words in total: ", stats.word_count,
        "Number of unique words: ", stats.unique_word_count, analyzer::get_fancy_percentage(stats.word_count, stats.unique_word_count),
        "Number of words appearing only once: ", stats.word_count_single_occurrence, analyzer::get_fancy_percentage(stats.unique_word_count, stats.word_count_single_occurrence),
        format_specific_stats,
    );

    println!("{}", formatted_stats);

    let mut stats_file =
        std::fs::File::create(&"analysis.txt").expect("Failed to create stats file");
    std::io::Write::write_all(&mut stats_file, formatted_stats.as_bytes())
        .expect("Failed to write stats file");

    let word_occurrence_list_formatted = stats
        .word_occurrence_list_sorted
        .into_iter()
        .fold(Vec::new(), |mut vec, x| {
            vec.push(x.0 + "\t" + &x.1.to_string());
            vec
        })
        .join("\n");

    let mut word_list_file =
        std::fs::File::create(&"word_list.csv").expect("Failed to create word list file");
    std::io::Write::write_all(
        &mut word_list_file,
        word_occurrence_list_formatted.as_bytes(),
    )
    .expect("Failed to write word list file");
}

fn run_tokenization(lines: &Vec<String>, dict: &JapaneseDictionary) -> Vec<String> {
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(dict);
    let mut morpheme_surfaces: Vec<String> = Default::default();
    for line in lines {
        let morphemes = match sudachi::analysis::Tokenize::tokenize(
            &tokenizer,
            line,
            dict_handler::get_mode(),
            false,
        ) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Line failed to tokenize `{}`\nError: `{}`", line, err);
                continue;
            }
        };
        for morpheme in morphemes.iter() {
            morpheme_surfaces.push(morpheme.surface().to_string());
        }
    }
    return morpheme_surfaces;
}

fn get_stats(
    lines: Vec<String>,
    morpheme_surfaces: Vec<String>,
    json_file_count: usize,
    json_dir_count: usize,
) -> AnalysisStats {
    let characters = morpheme_surfaces.join("");
    let filtered_morphemes = analyzer::filter_blacklisted(&morpheme_surfaces);

    let word_occurrence_list = analyzer::generate_occurrence_list(&filtered_morphemes);
    let characters_occurrence_list =
        analyzer::generate_occurrence_list(&characters.chars().collect());

    let word_occurrence_list_sorted = analyzer::sort_occurrence_list(&word_occurrence_list);
    let word_count_single_occurrence = analyzer::find_single_occurrences(&word_occurrence_list);

    let characters_count_single_occurrence =
        analyzer::find_single_occurrences(&characters_occurrence_list);
    let kanji_count_single_occurrence =
        analyzer::filter_non_kanji(&characters_count_single_occurrence);

    let japanese_characters = analyzer::filter_non_japanese(&characters.chars().collect());
    let kanji_characters = analyzer::filter_non_kanji(&characters.chars().collect());
    let mut unique_kanji_characters: Vec<char> = kanji_characters.clone();
    unique_kanji_characters.sort();
    unique_kanji_characters.dedup();

    let box_length = analyzer::get_avg_len(lines).unwrap_or_default();

    return AnalysisStats {
        char_count: japanese_characters.len(),
        kanji_count: kanji_characters.len(),
        unique_kanji_count: unique_kanji_characters.len(),
        kanji_count_single_occurrence: kanji_count_single_occurrence.len(),
        word_count: filtered_morphemes.len(),
        unique_word_count: word_occurrence_list_sorted.len(),
        word_count_single_occurrence: word_count_single_occurrence.len(),
        volume_count: json_dir_count,
        avg_volume_length: japanese_characters.len() / json_dir_count,
        page_count: json_file_count,
        avg_page_length: japanese_characters.len() / json_file_count,
        avg_box_length: box_length.average,
        shortest_box_length: box_length.shortest,
        longest_box_length: box_length.longest,
        box_count: box_length.length,

        word_occurrence_list_sorted: word_occurrence_list_sorted,
    };
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
    volume_count: usize,
    avg_volume_length: usize,
    page_count: usize,
    avg_page_length: usize,
    avg_box_length: usize,
    shortest_box_length: usize,
    longest_box_length: usize,
    box_count: usize,

    word_occurrence_list_sorted: Vec<(String, i32)>,
}
