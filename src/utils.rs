use serde_json;

use feature_pipe::config::PipeConfig;

use crate::predictor::BoosterPredictor;

const LGBM_MODEL_FILE: &'static str = "lgbm_model.txt";
const CONFIG_FILE: &'static str = "pipeline_config.json";

pub(crate) fn load_model() -> Result<BoosterPredictor, String> {
    BoosterPredictor::from_file(LGBM_MODEL_FILE)
        .map_err(|_| "No existing model found, or parsing failed!".to_string())
}

pub(crate) fn save_model(model_data: &str) -> Result<(), String> {
    std::fs::write(LGBM_MODEL_FILE, model_data)
        .map_err(|_| "Something went wrong while saving the model".to_string())
}

pub(crate) fn load_config() -> Result<PipeConfig, String> {
    let text = std::fs::read_to_string(CONFIG_FILE).map_err(|_| "reading in config file failed")?;

    serde_json::from_str::<PipeConfig>(&text).map_err(|_| "Parsing in json failed".to_string())
}

pub(crate) fn save_config(config: &PipeConfig) -> Result<(), String> {
    std::fs::write(
        CONFIG_FILE,
        serde_json::to_string_pretty(config).map_err(|_| "Parsing into json failed".to_string())?,
    )
    .map_err(|_| "Something went wrong while saving the config".to_string())
}
