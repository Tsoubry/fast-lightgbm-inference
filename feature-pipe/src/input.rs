use crate::config::{DataType, PipeConfig};
use crate::error::{FeatureError, NotFoundError, WrongValueError};
use crate::transformers::{make_nullable, string_to_f64, NULL};
use serde_json::Value;

pub fn convert_to_json_value(data: &str) -> Value {
    serde_json::from_str(data).expect("Couldn't parse json")
}

fn extract_value(value: &Value, data_type: &DataType) -> Option<f64> {
    match data_type {
        DataType::Float => value.as_f64(),
        DataType::Long => value.as_i64().map(|i| i as f64),
        DataType::Object => value.as_str().map(|v| string_to_f64(v)),
    }
}

pub fn transform_based_on_pipe(config: &PipeConfig, data: Value) -> Result<Vec<f64>, FeatureError> {
    let mut output: Vec<f64> = vec![];

    for feature in &config.features {
        match data.get(&feature.name) {
            Some(v) => {
                let feature_value = extract_value(v, &feature.datatype)
                    .ok_or(WrongValueError::new(&feature.name, &feature.datatype))?;

                if feature.nullable {
                    output.extend_from_slice(&make_nullable(feature_value));
                } else {
                    output.push(feature_value);
                }
            }
            None => {
                if feature.nullable {
                    output.extend_from_slice(&NULL)
                } else {
                    return Err(NotFoundError::new(&feature.name).into());
                }
            }
        };
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Feature;
    use serde_json::json;
    use serde_json::Map;

    #[test]
    fn test_transforming() {
        let config = PipeConfig {
            features: vec![
                Feature::new("A", DataType::Float, true),
                Feature::new("B", DataType::Long, false),
                Feature::new("C", DataType::Object, false),
                Feature::new("D", DataType::Float, true),
            ],
            is_prepared: true,
        };

        let mut json_map = Map::new();
        json_map.insert("A".to_string(), json!(100.4));
        json_map.insert("B".to_string(), json!(100));
        json_map.insert("C".to_string(), json!("Hello World"));

        let data_in = Value::Object(json_map);

        let result = transform_based_on_pipe(&config, data_in).unwrap();

        assert_eq!(result, vec![1., 100.4, 100., 7.982898449168957e18, 0., 0.,])
    }
}
