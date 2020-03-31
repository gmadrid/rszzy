use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

pub struct Version {
    pub version_number: u8,
    pub max_story_len: usize,

    pub packed_multiplier: u8,
}

impl Display for Version {
    fn fmt(&self, fmt: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Version {}", self.version_number)
    }
}

const V3: Version = Version {
    version_number: 3,
    max_story_len: 128 * 1024,
    packed_multiplier: 2,
};

const V5: Version = Version {
    version_number: 5,
    max_story_len: 256 * 1024,
    packed_multiplier: 4,
};

pub fn number_to_version(number: u8) -> Result<&'static Version> {
    let versions: Vec<&'static Version> = vec![&V3, &V5];

    versions
        .into_iter()
        .find(|v| v.version_number == number)
        .ok_or_else(|| anyhow!("Unknown (or unimplemented) version number: {}", number))
}
