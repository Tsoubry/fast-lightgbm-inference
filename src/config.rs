use crate::predictor::BoosterPredictor;
use feature_pipe::config::PipeConfig;

use log::warn;

use crate::utils::{load_config, load_model};

pub struct AppState {
    pub config: Option<PipeConfig>,
    pub model: Option<BoosterPredictor>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: get_config(),
            model: get_model(),
        }
    }
}

fn get_config() -> Option<PipeConfig> {
    match load_config() {
        Ok(config) => Some(config),
        Err(error) => {
            warn!("{}", error);
            None
        }
    }
}

fn get_model() -> Option<BoosterPredictor> {
    match load_model() {
        Ok(model) => Some(model),
        Err(error) => {
            warn!("{}", error);
            None
        }
    }
}
