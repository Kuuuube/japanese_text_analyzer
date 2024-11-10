pub fn get_args(args: Vec<String>) -> JapaneseTextAnalyzerArgs {
    let mut args_clone = args.clone();
    args_clone.remove(0);
    let mut japanese_text_analyzer_args = JapaneseTextAnalyzerArgs::default();

    for arg in args_clone {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--mokurojson" => {
                    japanese_text_analyzer_args.analysis_type = AnalysisType::MokuroJson;
                    japanese_text_analyzer_args.extension = ".json".to_string();
                },
                "--any" => {
                    japanese_text_analyzer_args.analysis_type = AnalysisType::Any;
                    japanese_text_analyzer_args.extension = "".to_string();
                },
                _ => {}
            }
        } else {
            japanese_text_analyzer_args.start_path = arg;
        }
    }
    return japanese_text_analyzer_args;
}

#[derive(Debug, Default)]
pub struct JapaneseTextAnalyzerArgs {
    pub start_path: String,
    pub analysis_type: AnalysisType,
    pub extension: String,
}

#[derive(Debug, Default)]
pub enum AnalysisType {
    #[default] MokuroJson,
    Any
}
