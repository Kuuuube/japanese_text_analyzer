pub fn get_args(args: Vec<String>) -> JapaneseTextAnalyzerArgs {
    let mut args_clone = args.clone();
    args_clone.remove(0);
    let mut japanese_text_analyzer_args = JapaneseTextAnalyzerArgs::default();

    for arg in args_clone {
        if arg.starts_with("--") {
            let split_arg = arg.split_once("=").unwrap_or_else(|| (&arg, ""));
            match split_arg.0 {
                "--mokurojson" => {
                    japanese_text_analyzer_args.analysis_type = AnalysisType::MokuroJson;
                    japanese_text_analyzer_args.extension = ".json".to_string();
                },
                "--mokuro" => {
                    japanese_text_analyzer_args.analysis_type = AnalysisType::Mokuro;
                    japanese_text_analyzer_args.extension = ".mokuro".to_string();
                },
                "--any" => {
                    japanese_text_analyzer_args.analysis_type = AnalysisType::Any;
                    japanese_text_analyzer_args.extension = split_arg.1.to_string();
                },
                _ => {}
            }
        } else {
            japanese_text_analyzer_args.start_path = arg;
        }
    }
    return japanese_text_analyzer_args;
}

#[derive(Debug)]
pub struct JapaneseTextAnalyzerArgs {
    pub start_path: String,
    pub analysis_type: AnalysisType,
    pub extension: String,
}

impl JapaneseTextAnalyzerArgs {
    fn default() -> Self {
        JapaneseTextAnalyzerArgs {
            start_path: "".to_string(),
            analysis_type: AnalysisType::MokuroJson,
            extension: ".json".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AnalysisType {
    MokuroJson,
    Mokuro,
    Any
}
