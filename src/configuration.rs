use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    /// Time resolution in fractional hours
    pub resolution: Option<f64>,
    /// Pay for an hour
    pub pay: Option<f64>,
    /// Maximum work hours per day
    pub max_hours: Option<u32>,
}

impl Configuration {
    pub fn update(&mut self, configuration: Configuration) {
        if let Some(resolution) = configuration.resolution {
            self.resolution = Some(resolution);
        }
        if let Some(pay) = configuration.pay {
            self.pay = Some(pay);
        }
        if let Some(max_hours) = configuration.max_hours {
            self.max_hours = Some(max_hours);
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            resolution: Some(0.25),
            pay: None,
            max_hours: None,
        }
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(resolution) = self.resolution {
            writeln!(f, "Resolution: {} hours", resolution)?;
        }
        if let Some(pay) = self.pay {
            writeln!(f, "Payment per hour: {}", pay)?
        };
        if let Some(max_hours) = self.max_hours {
            writeln!(f, "Maximum work time: {} hours", max_hours)?
        };
        Ok(())
    }
}
