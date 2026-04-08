use std::collections::{HashMap, HashSet};

use crate::{analyzer, args_parser::{AnalysisType, JapaneseTextAnalyzerArgs}};

pub fn get_stats(
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
pub struct AnalysisStats {
    pub char_count: u64,
    pub kanji_count: u64,
    pub unique_kanji: HashSet<char>,
    pub word_count: u64,
    pub unique_words: HashSet<String>,
    pub volume_count: usize,
    pub avg_volume_length: f64,
    pub page_count: usize,
    pub avg_page_length: f64,
    pub avg_box_length: f64,
    pub shortest_box_length: usize,
    pub longest_box_length: usize,
    pub box_count: u64,

    pub word_list_raw: Vec<String>,
    pub kanji_occurrence_list: HashMap<char, u64>,
    pub word_occurrence_list: HashMap<String, u64>,
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
    pub fn combine(&mut self, stats2: AnalysisStats) {
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

    pub fn format_fancy(&mut self, parsed_args: JapaneseTextAnalyzerArgs) -> String {
        let format_specific_stats = match parsed_args.analysis_type {
            AnalysisType::MokuroJson => format!(
                "{}{:.0} ({} total volumes)\n{}{:.0} ({} total pages)\n{}{:.0} (shortest: {}) (longest: {}) ({} total textboxes)",
                "Average volume length in characters: ",
                self.avg_volume_length,
                self.volume_count,
                "Average page length in characters: ",
                self.avg_page_length,
                self.page_count,
                "Average textbox length in characters: ",
                self.avg_box_length,
                self.shortest_box_length,
                self.longest_box_length,
                self.box_count
            ),
            AnalysisType::Any => "".to_string(),
            AnalysisType::Mokuro => format!(
                "{}{} (shortest: {}) (longest: {}) ({} total textboxes)",
                "Average textbox length in characters: ",
                self.avg_box_length,
                self.shortest_box_length,
                self.longest_box_length,
                self.box_count
            ),
        };

        let unique_word_count = self.unique_words.len();
        let unique_kanji_count = self.unique_kanji.len();
        let word_count_single_occurrence =
            analyzer::find_single_occurrences(&self.word_occurrence_list).len();
        let kanji_count_single_occurrence =
            analyzer::find_single_occurrences(&self.kanji_occurrence_list).len();

        let formatted_stats = format!(
            "{}\n{}\n{}{}\n{}{}\n{}{}\n{}{} ({} of unique kanji)\n{}{}\n{}{} ({} of all words)\n{}{} ({} of unique words)\n{}",
            parsed_args.start_path,
            "----------------------------------------------------------------------------",
            "Number of Japanese characters: ",
            self.char_count,
            "Number of kanji characters: ",
            self.kanji_count,
            "Number of unique kanji: ",
            unique_kanji_count,
            "Number of unique kanji appearing only once: ",
            kanji_count_single_occurrence,
            analyzer::get_fancy_percentage(
                unique_kanji_count as f64,
                kanji_count_single_occurrence as f64
            ),
            "Number of words in total: ",
            self.word_count,
            "Number of unique words: ",
            unique_word_count,
            analyzer::get_fancy_percentage(self.word_count as f64, unique_word_count as f64),
            "Number of words appearing only once: ",
            word_count_single_occurrence,
            analyzer::get_fancy_percentage(
                unique_word_count as f64,
                word_count_single_occurrence as f64
            ),
            format_specific_stats,
        );

        return formatted_stats;
    }
}
