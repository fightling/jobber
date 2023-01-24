use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Duration {
    Zero,
    HM { hours: u32, minutes: u32 },
}

impl Duration {
    pub fn parse(d: String) -> Self {
        Self::parse_hm(&d).or(Self::parse_hours(&d).or(Self::parse_hm2(&d)))
    }
    pub fn or(self, d: Self) -> Self {
        match self {
            Self::Zero => d,
            _ => self,
        }
    }
    /// parse time "HH:MM"
    fn parse_hm(d: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap(),
                minutes: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::Zero
    }
    fn parse_hours(d: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})[,.](\d{1,2})$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap(),
                minutes: (format!(".{}", cap[2].to_string()).parse::<f64>().unwrap() * 60f64)
                    as u32,
            };
        }
        let re = Regex::new(r"^[,.](\d{1,2})$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: 0,
                minutes: (format!(".{}", cap[1].to_string()).parse::<f64>().unwrap() * 60f64)
                    as u32,
            };
        }
        let re = Regex::new(r"^(\d{1,2})$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap(),
                minutes: 0,
            };
        }
        Self::Zero
    }
    fn parse_hm2(d: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})h(\d{1,2})m$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap(),
                minutes: cap[2].parse::<u32>().unwrap(),
            };
        }
        let re = Regex::new(r"^(\d{1,2})m$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: 0,
                minutes: cap[1].parse::<u32>().unwrap(),
            };
        }
        let re = Regex::new(r"^(\d{1,2})h$").unwrap();
        for cap in re.captures_iter(d) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap(),
                minutes: 0,
            };
        }
        Self::Zero
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Duration::HM { hours, minutes } => {
                write!(f, "{}", *hours as f64 + *minutes as f64 / 60f64)
            }
            _ => write!(f, "0"),
        }
    }
}

#[test]
fn test_duration() {
    assert_eq!(
        Duration::parse("2:30".to_string()),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2.5".to_string()),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse(".25".to_string()),
        Duration::HM {
            hours: 0,
            minutes: 15
        }
    );
    assert_eq!(
        Duration::parse(".5".to_string()),
        Duration::HM {
            hours: 0,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2".to_string()),
        Duration::HM {
            hours: 2,
            minutes: 0
        }
    );
    assert_eq!(
        Duration::parse("2h30m".to_string()),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2h".to_string()),
        Duration::HM {
            hours: 2,
            minutes: 0
        }
    );
    assert_eq!(
        Duration::parse("15m".to_string()),
        Duration::HM {
            hours: 0,
            minutes: 15
        }
    );
}
