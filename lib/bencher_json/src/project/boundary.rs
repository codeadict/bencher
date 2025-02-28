use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonBoundaries(pub Vec<JsonBoundary>);

crate::from_vec!(JsonBoundaries[JsonBoundary]);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonBoundary {
    pub lower_limit: Option<OrderedFloat<f64>>,
    pub upper_limit: Option<OrderedFloat<f64>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum JsonLimit {
    Lower,
    Upper,
}
