use std::str::from_utf8;
use std::sync::RwLock;

use actix_web::{error, post, web, Error, HttpResponse};
use feature_pipe::config::PipeConfig;
use feature_pipe::input::transform_based_on_pipe;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::AppState;
use crate::predictor::BoosterPredictor;
use crate::utils::{save_config, save_model};

fn unprocessable(msg: String) -> Error {
    error::ErrorUnprocessableEntity(msg)
}

#[post("/config")]
pub async fn update_config(
    data: web::Data<RwLock<AppState>>,
    item: web::Json<PipeConfig>,
) -> Result<HttpResponse, Error> {
    let web::Json(new_config) = item;

    if let Err(error) = save_config(&new_config) {
        return Err(unprocessable(error));
    }

    {
        let mut current_state = data.write().unwrap();
        current_state.config = Some(new_config)
    }

    Ok(HttpResponse::Ok().finish())
}

#[post("/model")]
pub async fn update_model(
    data: web::Data<RwLock<AppState>>,
    mut content: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = content.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let model_string = from_utf8(&bytes).map_err(|e| {
        unprocessable(format!(
            "Unable to process bytes to string: {}",
            e.to_string()
        ))
    })?;

    let new_model = BoosterPredictor::from_string(model_string)
        .map_err(|e| unprocessable(format!("Unable to load Lightgbm model: {}", e.to_string())))?;

    save_model(model_string).map_err(|e| unprocessable(e))?;

    {
        let mut current_state = data.write().unwrap();
        current_state.model = Some(new_model)
    }

    Ok(HttpResponse::Ok().finish())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Prediction {
    prediction: Vec<f64>,
}

#[post("/predict")]
pub async fn predict(
    data: web::Data<RwLock<AppState>>,
    item: web::Json<Value>,
) -> Result<HttpResponse, Error> {
    let current_state = data.read().unwrap();
    let current_config = current_state
        .config
        .as_ref()
        .ok_or(|| ())
        .map_err(|_| error::ErrorBadRequest("No config loaded".to_string()))?;
    let current_model = current_state
        .model
        .as_ref()
        .ok_or(|| ())
        .map_err(|_| error::ErrorBadRequest("No model loaded".to_string()))?;

    let features_transformed = transform_based_on_pipe(current_config, item.0).unwrap();
    let prediction = current_model.predict(vec![features_transformed]).unwrap();

    drop(current_state);

    let output = serde_json::to_string(&Prediction {
        prediction: prediction.first().unwrap().clone(),
    })
    .unwrap();

    Ok(HttpResponse::Ok().body(output))
}
