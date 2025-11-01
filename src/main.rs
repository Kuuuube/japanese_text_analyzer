use std::collections::{HashMap, HashSet};

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

    println!("Loading tokenizer dictionary");
    let start_time = std::time::Instant::now();
    let dict = dict_handler::make_sudachi_dict().expect("Failed to load tokenizer dictionary");
    println!("Dictionary loaded ({}ms)", start_time.elapsed().as_millis());

    println!("Processing files, running tokenizer, and analyzing results");
    let start_time = std::time::Instant::now();
    let mut stats: AnalysisStats = Default::default();
    for file in files {
        let maybe_lines = match parsed_args.analysis_type {
            AnalysisType::MokuroJson => file_handler::get_json_file_data(file),
            AnalysisType::Mokuro => file_handler::get_mokuro_file_data(file),
            AnalysisType::Any => file_handler::get_plain_file_data(file),
        };
        if let Some(lines) = maybe_lines
            && lines.len() > 0
        {
            let morpheme_surfaces = run_tokenization(&lines, &dict);
            let new_stats = get_stats(lines, morpheme_surfaces, file_count, dir_count);
            stats.combine(new_stats);
        }
    }
    println!(
        "Tokenizer and analysis finished ({}ms)",
        start_time.elapsed().as_millis()
    );

    let format_specific_stats = match parsed_args.analysis_type {
        AnalysisType::MokuroJson => format!(
            "{}{:.0} ({} total volumes)\n{}{:.0} ({} total pages)\n{}{:.0} (shortest: {}) (longest: {}) ({} total textboxes)",
            "Average volume length in characters: ",
            stats.avg_volume_length,
            stats.volume_count,
            "Average page length in characters: ",
            stats.avg_page_length,
            stats.page_count,
            "Average textbox length in characters: ",
            stats.avg_box_length,
            stats.shortest_box_length,
            stats.longest_box_length,
            stats.box_count
        ),
        AnalysisType::Any => "".to_string(),
        AnalysisType::Mokuro => format!(
            "{}{} (shortest: {}) (longest: {}) ({} total textboxes)",
            "Average textbox length in characters: ",
            stats.avg_box_length,
            stats.shortest_box_length,
            stats.longest_box_length,
            stats.box_count
        ),
    };

    let unique_word_count = stats.unique_words.len();
    let unique_kanji_count = stats.unique_kanji.len();
    let word_count_single_occurrence = stats.words_single_occurrence.len();
    let kanji_count_single_occurrence = stats.kanji_single_occurrence.len();

    let formatted_stats = format!(
        "{}\n{}\n{}{}\n{}{}\n{}{}\n{}{} ({} of unique kanji)\n{}{}\n{}{} ({} of all words)\n{}{} ({} of unique words)\n{}",
        parsed_args.start_path,
        "----------------------------------------------------------------------------",
        "Number of Japanese characters: ",
        stats.char_count,
        "Number of kanji characters: ",
        stats.kanji_count,
        "Number of unique kanji: ",
        unique_kanji_count,
        "Number of unique kanji appearing only once: ",
        kanji_count_single_occurrence,
        analyzer::get_fancy_percentage(unique_kanji_count, kanji_count_single_occurrence),
        "Number of words in total: ",
        stats.word_count,
        "Number of unique words: ",
        unique_word_count,
        analyzer::get_fancy_percentage(stats.word_count, unique_word_count),
        "Number of words appearing only once: ",
        word_count_single_occurrence,
        analyzer::get_fancy_percentage(unique_word_count, word_count_single_occurrence),
        format_specific_stats,
    );

    println!("{}", formatted_stats);

    let mut stats_file =
        std::fs::File::create(&"analysis.txt").expect("Failed to create stats file");
    std::io::Write::write_all(&mut stats_file, formatted_stats.as_bytes())
        .expect("Failed to write stats file");

    let word_occurrence_list_formatted = analyzer::sort_occurrence_list(stats.word_occurrence_list)
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

    let mut word_list_raw_file =
        std::fs::File::create(&"word_list_raw.csv").expect("Failed to create word list raw file");
    std::io::Write::write_all(
        &mut word_list_raw_file,
        stats.word_list_raw.join("\n").as_bytes(),
    )
    .expect("Failed to write word list raw file");
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

    let words_single_occurrence = analyzer::find_single_occurrences(&word_occurrence_list);

    let characters_count_single_occurrence =
        analyzer::find_single_occurrences(&characters_occurrence_list);
    let kanji_single_occurrence = analyzer::filter_non_kanji(&characters_count_single_occurrence);

    let japanese_characters = analyzer::filter_non_japanese(&characters.chars().collect());
    let kanji_characters: Vec<char> = analyzer::filter_non_kanji(&characters.chars().collect());
    let mut unique_kanji_characters: Vec<char> = kanji_characters.clone();
    unique_kanji_characters.sort();
    unique_kanji_characters.dedup();

    let box_length = analyzer::get_avg_len(lines).unwrap_or_default();

    return AnalysisStats {
        char_count: japanese_characters.len(),
        kanji_count: kanji_characters.len(),
        unique_kanji: HashSet::from_iter(unique_kanji_characters.iter().cloned()),
        kanji_single_occurrence,
        word_count: filtered_morphemes.len(),
        unique_words: HashSet::from_iter(word_occurrence_list.iter().map(|x| x.0.to_owned())),
        words_single_occurrence,
        volume_count: json_dir_count,
        avg_volume_length: japanese_characters.len() as f64 / json_dir_count as f64,
        page_count: json_file_count,
        avg_page_length: japanese_characters.len() as f64 / json_file_count as f64,
        avg_box_length: box_length.average as f64,
        shortest_box_length: box_length.shortest,
        longest_box_length: box_length.longest,
        box_count: box_length.length,

        word_list_raw: filtered_morphemes,
        word_occurrence_list: word_occurrence_list,
    };
}

#[derive(Debug)]
struct AnalysisStats {
    char_count: usize,
    kanji_count: usize,
    unique_kanji: HashSet<char>,
    kanji_single_occurrence: HashSet<char>,
    word_count: usize,
    unique_words: HashSet<String>,
    words_single_occurrence: HashSet<String>,
    volume_count: usize,
    avg_volume_length: f64,
    page_count: usize,
    avg_page_length: f64,
    avg_box_length: f64,
    shortest_box_length: usize,
    longest_box_length: usize,
    box_count: usize,

    word_list_raw: Vec<String>,
    word_occurrence_list: HashMap<String, i32>,
}

impl Default for AnalysisStats {
    fn default() -> Self {
        Self {
            char_count: Default::default(),
            kanji_count: Default::default(),
            unique_kanji: Default::default(),
            kanji_single_occurrence: Default::default(),
            word_count: Default::default(),
            unique_words: Default::default(),
            words_single_occurrence: Default::default(),
            volume_count: Default::default(),
            avg_volume_length: Default::default(),
            page_count: Default::default(),
            avg_page_length: Default::default(),
            avg_box_length: Default::default(),
            shortest_box_length: usize::MAX,
            longest_box_length: Default::default(),
            box_count: Default::default(),
            word_list_raw: Default::default(),
            word_occurrence_list: Default::default(),
        }
    }
}

impl AnalysisStats {
    fn combine(&mut self, stats2: AnalysisStats) {
        *self = AnalysisStats {
            char_count: self.char_count + stats2.char_count,
            kanji_count: self.kanji_count + stats2.kanji_count,
            unique_kanji: self
                .unique_kanji
                .union(&stats2.unique_kanji)
                .map(|x| x.to_owned())
                .collect(),
            kanji_single_occurrence: self
                .kanji_single_occurrence
                .symmetric_difference(&stats2.kanji_single_occurrence)
                .map(|x| x.to_owned())
                .collect(),
            word_count: self.word_count + stats2.word_count,
            unique_words: self
                .unique_words
                .union(&stats2.unique_words)
                .map(|x| x.to_owned())
                .collect(),
            words_single_occurrence: self
                .words_single_occurrence
                .symmetric_difference(&stats2.words_single_occurrence)
                .map(|x| x.to_owned())
                .collect(),
            volume_count: stats2.volume_count,
            avg_volume_length: self.avg_volume_length + stats2.avg_volume_length,
            page_count: stats2.page_count,
            avg_page_length: self.avg_page_length + stats2.avg_page_length,
            avg_box_length: f64::max(
                (self.avg_box_length * self.box_count as f64
                    + stats2.avg_box_length * stats2.box_count as f64)
                    / (self.box_count + stats2.box_count) as f64,
                0.0, // cure potential NaN corruption by overriding NaNs with 0.0
            ),
            shortest_box_length: analyzer::bounded_min(self.shortest_box_length, stats2.shortest_box_length, 1),
            longest_box_length: usize::max(self.longest_box_length, stats2.longest_box_length),
            box_count: self.box_count + stats2.box_count,
            word_list_raw: self
                .word_list_raw
                .iter()
                .chain(&stats2.word_list_raw)
                .cloned()
                .collect(),
            word_occurrence_list: self.word_occurrence_list.iter().fold(
                stats2.word_occurrence_list,
                |mut hashmap, x| {
                    if let Some(mut_map_ref) = hashmap.get_mut(x.0.as_str()) {
                        *mut_map_ref += x.1;
                    }
                    hashmap
                },
            ),
        };
    }
}
