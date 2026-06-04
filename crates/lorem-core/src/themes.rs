//! Themed corpora. Each theme is a plain-text corpus that seeds its own
//! Markov chain.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Classic,
    Tech,
    Pirate,
    Corporate,
    Cosmic,
}

impl Theme {
    pub const ALL: [Theme; 5] = [
        Theme::Classic,
        Theme::Tech,
        Theme::Pirate,
        Theme::Corporate,
        Theme::Cosmic,
    ];

    pub fn id(&self) -> &'static str {
        match self {
            Theme::Classic => "classic",
            Theme::Tech => "tech",
            Theme::Pirate => "pirate",
            Theme::Corporate => "corporate",
            Theme::Cosmic => "cosmic",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Theme::Classic => "Classic Latin",
            Theme::Tech => "Tech Startup",
            Theme::Pirate => "Pirate",
            Theme::Corporate => "Corporate Buzzword",
            Theme::Cosmic => "Cosmic",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Theme::Classic => "Traditional lorem ipsum pseudo-Latin",
            Theme::Tech => "Startup and developer jargon",
            Theme::Pirate => "High-seas adventure speak",
            Theme::Corporate => "Boardroom buzzword bingo",
            Theme::Cosmic => "Spacefaring nebula prose",
        }
    }

    pub fn corpus(&self) -> &'static str {
        match self {
            Theme::Classic => include_str!("themes/classic.txt"),
            Theme::Tech => include_str!("themes/tech.txt"),
            Theme::Pirate => include_str!("themes/pirate.txt"),
            Theme::Corporate => include_str!("themes/corporate.txt"),
            Theme::Cosmic => include_str!("themes/cosmic.txt"),
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(Theme::Classic),
            "tech" => Ok(Theme::Tech),
            "pirate" => Ok(Theme::Pirate),
            "corporate" => Ok(Theme::Corporate),
            "cosmic" => Ok(Theme::Cosmic),
            other => Err(format!(
                "unknown theme '{other}' (expected one of: classic, tech, pirate, corporate, cosmic)"
            )),
        }
    }
}
