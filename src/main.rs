use std::collections::{HashMap, HashSet};

use args_parser::AnalysisType;
use sudachi::{
    analysis::stateless_tokenizer::StatelessTokenizer, dic::dictionary::JapaneseDictionary,
};

mod analyzer;
mod args_parser;
mod dict_handler;
mod file_handler;
mod tests;
mod utf8_bufreader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = args_parser::get_args(args);
    if parsed_args.help {
        println!(include_str!("help_text.txt"));
        return;
    }

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
    let tokenizer = sudachi::analysis::stateless_tokenizer::StatelessTokenizer::new(&dict);
    println!("Dictionary loaded ({}ms)", start_time.elapsed().as_millis());

    println!("Processing files, running tokenizer, and analyzing results");
    let start_time = std::time::Instant::now();
    let mut stats = AnalysisStats::default();
    let mut word_list_raw_file =
        std::fs::File::create(&"word_list_raw.csv").expect("Failed to create word list raw file");
    for file_path in files {
        match parsed_args.analysis_type {
            AnalysisType::MokuroJson => {
                let lines = file_handler::get_json_file_data(file_path);
                let morpheme_surfaces = run_tokenization(&lines, &tokenizer);
                let new_stats = get_stats(lines, morpheme_surfaces, file_count, dir_count);
                std::io::Write::write_all(
                    &mut word_list_raw_file,
                    (new_stats.word_list_raw.join("\n") + "\n").as_bytes(),
                )
                .expect("Failed to write word list raw file");
                stats.combine(new_stats);
            }
            AnalysisType::Mokuro => {
                let lines = file_handler::get_mokuro_file_data(file_path);
                let morpheme_surfaces = run_tokenization(&lines, &tokenizer);
                let new_stats = get_stats(lines, morpheme_surfaces, file_count, dir_count);
                std::io::Write::write_all(
                    &mut word_list_raw_file,
                    (new_stats.word_list_raw.join("\n") + "\n").as_bytes(),
                )
                .expect("Failed to write word list raw file");
                stats.combine(new_stats);
            }
            AnalysisType::Any => {
                if let Ok(buffered_plain_line_reader) =
                    file_handler::BufferedPlainLineReader::new(&file_path)
                {
                    for lines in buffered_plain_line_reader {
                        let morpheme_surfaces = run_tokenization(&lines, &tokenizer);
                        let new_stats = get_stats(lines, morpheme_surfaces, file_count, dir_count);
                        std::io::Write::write_all(
                            &mut word_list_raw_file,
                            (new_stats.word_list_raw.join("\n") + "\n").as_bytes(),
                        )
                        .expect("Failed to write word list raw file");
                        stats.combine(new_stats);
                    }
                }
            }
        };
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
    let word_count_single_occurrence =
        analyzer::find_single_occurrences(&stats.word_occurrence_list).len();
    let kanji_count_single_occurrence =
        analyzer::find_single_occurrences(&stats.kanji_occurrence_list).len();

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
        analyzer::get_fancy_percentage(
            unique_kanji_count as f64,
            kanji_count_single_occurrence as f64
        ),
        "Number of words in total: ",
        stats.word_count,
        "Number of unique words: ",
        unique_word_count,
        analyzer::get_fancy_percentage(stats.word_count as f64, unique_word_count as f64),
        "Number of words appearing only once: ",
        word_count_single_occurrence,
        analyzer::get_fancy_percentage(
            unique_word_count as f64,
            word_count_single_occurrence as f64
        ),
        format_specific_stats,
    );

    println!("{}", formatted_stats);

    let mut stats_file =
        std::fs::File::create(&"analysis.txt").expect("Failed to create stats file");
    std::io::Write::write_all(&mut stats_file, formatted_stats.as_bytes())
        .expect("Failed to write stats file");

    let word_occurrence_list_formatted =
        analyzer::sort_occurrence_list(stats.word_occurrence_list.clone())
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

    let kanji_occurrence_list_formatted =
        analyzer::sort_occurrence_list(stats.kanji_occurrence_list.clone())
            .into_iter()
            .fold(Vec::new(), |mut vec, x| {
                vec.push(x.0 + "\t" + &x.1.to_string());
                vec
            })
            .join("\n");

    let mut kanji_list_file =
        std::fs::File::create(&"kanji_list.csv").expect("Failed to create kanji list file");
    std::io::Write::write_all(
        &mut kanji_list_file,
        kanji_occurrence_list_formatted.as_bytes(),
    )
    .expect("Failed to write kanji list file");
}

fn run_tokenization(
    lines: &Vec<String>,
    tokenizer: &StatelessTokenizer<&JapaneseDictionary>,
) -> Vec<String> {
    let mut morpheme_surfaces: Vec<String> = Default::default();
    for line in lines {
        let morphemes = match sudachi::analysis::Tokenize::tokenize(
            tokenizer,
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

    let japanese_characters = analyzer::filter_non_japanese(&characters.chars().collect());
    let kanji_characters: Vec<char> = analyzer::filter_non_kanji(&characters.chars().collect());
    let kanji_occurrence_list = analyzer::generate_occurrence_list(&kanji_characters);
    let mut unique_kanji_characters: Vec<char> = kanji_characters.clone();
    unique_kanji_characters.sort();
    unique_kanji_characters.dedup();

    let box_length = analyzer::get_avg_len(lines).unwrap_or_default();

    return AnalysisStats {
        char_count: japanese_characters.len() as u64,
        kanji_count: kanji_characters.len() as u64,
        unique_kanji: HashSet::from_iter(unique_kanji_characters.iter().cloned()),
        word_count: filtered_morphemes.len() as u64,
        unique_words: HashSet::from_iter(word_occurrence_list.iter().map(|x| x.0.to_owned())),
        volume_count: json_dir_count,
        avg_volume_length: japanese_characters.len() as f64 / json_dir_count as f64,
        page_count: json_file_count,
        avg_page_length: japanese_characters.len() as f64 / json_file_count as f64,
        avg_box_length: box_length.average as f64,
        shortest_box_length: box_length.shortest,
        longest_box_length: box_length.longest,
        box_count: box_length.length as u64,

        word_list_raw: filtered_morphemes,
        kanji_occurrence_list: kanji_occurrence_list,
        word_occurrence_list: word_occurrence_list,
    };
}

#[derive(Debug)]
struct AnalysisStats {
    char_count: u64,
    kanji_count: u64,
    unique_kanji: HashSet<char>,
    word_count: u64,
    unique_words: HashSet<String>,
    volume_count: usize,
    avg_volume_length: f64,
    page_count: usize,
    avg_page_length: f64,
    avg_box_length: f64,
    shortest_box_length: usize,
    longest_box_length: usize,
    box_count: u64,

    word_list_raw: Vec<String>,
    kanji_occurrence_list: HashMap<char, u64>,
    word_occurrence_list: HashMap<String, u64>,
}

impl Default for AnalysisStats {
    fn default() -> Self {
        Self {
            char_count: Default::default(),
            kanji_count: Default::default(),
            unique_kanji: Default::default(),
            word_count: Default::default(),
            unique_words: Default::default(),
            volume_count: Default::default(),
            avg_volume_length: Default::default(),
            page_count: Default::default(),
            avg_page_length: Default::default(),
            avg_box_length: Default::default(),
            shortest_box_length: usize::MAX,
            longest_box_length: Default::default(),
            box_count: Default::default(),
            word_list_raw: Default::default(),
            kanji_occurrence_list: Default::default(),
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
            word_count: self.word_count + stats2.word_count,
            unique_words: self
                .unique_words
                .union(&stats2.unique_words)
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
            shortest_box_length: analyzer::bounded_min(
                self.shortest_box_length,
                stats2.shortest_box_length,
                1,
            ),
            longest_box_length: usize::max(self.longest_box_length, stats2.longest_box_length),
            box_count: self.box_count + stats2.box_count,
            word_list_raw: vec![],
            kanji_occurrence_list: analyzer::merge_hashmap(
                stats2.kanji_occurrence_list,
                &self.kanji_occurrence_list,
            ),
            word_occurrence_list: analyzer::merge_hashmap(
                stats2.word_occurrence_list,
                &self.word_occurrence_list,
            ),
        };
    }
}
