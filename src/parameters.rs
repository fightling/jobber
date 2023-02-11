use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameters {
    /// Time resolution in fractional hours
    pub resolution: f64,
    /// Pay for an hour
    pub pay: Option<f64>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            resolution: 0.25,
            pay: None,
        }
    }
}
