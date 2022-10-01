use serde::{Deserialize, Serialize};

use crate::error::{FeatureError, NotPreparedError};
use crate::input::{convert_to_json_value, transform_based_on_pipe};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataType {
    Float,
    Long,
    Object,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub datatype: DataType,
    pub nullable: bool,
}

impl Feature {
    pub fn new(name: &str, datatype: DataType, nullable: bool) -> Self {
        Self {
            name: name.to_string(),
            datatype,
            nullable,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeConfig {
    pub features: Vec<Feature>,
    #[serde(skip_serializing, skip_deserializing, default)]
    pub is_prepared: bool,
}

impl PipeConfig {
    fn sort_features(&mut self) {
        self.features
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    }

    pub fn prepare(&mut self) {
        self.sort_features();
        self.is_prepared = true;
    }

    pub fn transform(self, data: &str) -> Result<Vec<f64>, FeatureError> {
        if !self.is_prepared {
            return Err(NotPreparedError.into());
        }

        let v = convert_to_json_value(data);

        transform_based_on_pipe(&self, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let mut config = PipeConfig {
            features: vec![
                Feature::new("B", DataType::Long, false),
                Feature::new("D", DataType::Float, true),
                Feature::new("C", DataType::Object, false),
                Feature::new("A", DataType::Float, true),
            ],
            is_prepared: true,
        };

        config.sort_features();
        let result: Vec<String> = config.features.into_iter().map(|f| f.name).collect();

        assert_eq!(
            result,
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string()
            ]
        );
    }

    #[test]
    fn test_end_to_end_transforming() {
        let config_data = r#"
        {
            "features": [
                {
                    "name": "A",
                    "datatype": "Float",
                    "nullable": true
                },
                {
                    "name": "B",
                    "datatype": "Long",
                    "nullable": false
                },
                {
                    "name": "C",
                    "datatype": "Object",
                    "nullable": false
                },
                {
                    "name": "D",
                    "datatype": "Float",
                    "nullable": true
                }
            ]
        }"#;

        let data = r#"
        {
            "A": 100.4,
            "B": 100,
            "C": "Hello World"
        }"#;

        let mut config: PipeConfig = serde_json::from_str(config_data).unwrap();
        config.prepare();

        let result = config.transform(data).unwrap();

        assert_eq!(result, vec![1., 100.4, 100., 7.982898449168957e18, 0., 0.,]);
    }
}
