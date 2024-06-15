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

    analyzer::analyze_morphemes(&morpheme_surfaces);

    println!("{}", morpheme_surfaces.join("|"));
    println!("Morphemes found: {}", morpheme_surfaces.len());
}
