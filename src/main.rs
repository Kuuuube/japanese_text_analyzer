use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex, RwLock},
};

use args_parser::AnalysisType;
use rayon::iter::{ParallelBridge, ParallelIterator};
use sudachi::{
    analysis::stateless_tokenizer::StatelessTokenizer, dic::dictionary::JapaneseDictionary,
};

use crate::{stats_handler::AnalysisStats, type_extensions::MutexExtensions};

mod analyzer;
mod args_parser;
mod dict_handler;
mod file_handler;
mod stats_handler;
mod tests;
mod type_extensions;
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
    let stats = Arc::new(Mutex::new(stats_handler::AnalysisStats::default()));
    let word_list_raw_file = Arc::new(RwLock::new(
        std::fs::File::create(&"word_list_raw.csv").expect("Failed to create word list raw file"),
    ));

    let process_closure = |lines| {
        process_lines(
            lines,
            &tokenizer,
            word_list_raw_file.clone(),
            stats.clone(),
            file_count,
            dir_count,
        );
    };

    // overhead of spawning a thread generally is not worth it for running over a lot of small files
    for file_path in files {
        match parsed_args.analysis_type {
            AnalysisType::MokuroJson => {
                let lines = file_handler::get_json_file_data(&file_path);
                process_closure(lines);
            }
            AnalysisType::Mokuro => {
                let lines = file_handler::get_mokuro_file_data(&file_path);
                process_closure(lines);
            }
            AnalysisType::Any => {
                if let Ok(buffered_plain_line_reader) =
                    file_handler::BufferedPlainLineReader::new(&file_path)
                {
                    if parsed_args.singlethreaded {
                        buffered_plain_line_reader.for_each(process_closure);
                    } else {
                        buffered_plain_line_reader
                            .par_bridge()
                            .for_each(process_closure);
                    }
                }
            }
        };
    }
    println!(
        "Tokenizer and analysis finished ({}ms)",
        start_time.elapsed().as_millis()
    );

    let mut stats = stats.lock().expect("Failed to get stats reader");

    let formatted_stats = stats.format_fancy(parsed_args);

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

fn process_lines(
    lines: Vec<String>,
    tokenizer: &StatelessTokenizer<&JapaneseDictionary>,
    word_list_raw_file: Arc<RwLock<File>>,
    stats: Arc<Mutex<AnalysisStats>>,
    file_count: usize,
    dir_count: usize,
) {
    let morpheme_surfaces = run_tokenization(&lines, &tokenizer);
    let new_stats = stats_handler::get_stats(lines, morpheme_surfaces, file_count, dir_count);
    {
        let word_list_raw_file_lock = &mut word_list_raw_file
            .write()
            .expect("Failed to get word_list_raw writer");
        std::io::Write::write_all(
            word_list_raw_file_lock.by_ref(),
            (new_stats.word_list_raw.join("\n") + "\n").as_bytes(),
        )
        .expect("Failed to write word list raw file");
    }
    {
        println!("arriving at lock");
        stats
            .replace_with(|value| value.combine(new_stats))
            .unwrap();
        println!("finished");
    }
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
