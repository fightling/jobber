use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    /// Time resolution in fractional hours
    pub resolution: f64,
    /// Pay for an hour
    pub pay: Option<f64>,
    /// Maximum work hours per day
    pub max_hours: Option<u32>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            resolution: 0.25,
            pay: None,
            max_hours: None,
        }
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tResolution: {} hours", self.resolution)?;
        if let Some(pay) = self.pay {
            writeln!(f, "\tPayment per hour: {}", pay)?
        };
        if let Some(max_hours) = self.max_hours {
            writeln!(f, "\tMaximum work time: {} hours", max_hours)?
        };
        Ok(())
    }
}
