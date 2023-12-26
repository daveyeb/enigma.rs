use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, PartialEq)]
pub enum Rotor {
    #[strum(ascii_case_insensitive)]
    I,
    #[strum(ascii_case_insensitive)]
    II,
    #[strum(ascii_case_insensitive)]
    III,
    #[strum(ascii_case_insensitive)]
    IV,
    #[strum(ascii_case_insensitive)]
    V,
    #[strum(ascii_case_insensitive)]
    VI,
    #[strum(ascii_case_insensitive)]
    VII,
    #[strum(ascii_case_insensitive)]
    VIII,
    #[strum(ascii_case_insensitive)]
    BETA,
    #[strum(ascii_case_insensitive)]
    GAMMA,
}

#[derive(Debug, Default, Clone, EnumString, PartialEq)]
pub enum Reflector {
    #[default]
    #[strum(ascii_case_insensitive)]
    UKWB,
    #[strum(ascii_case_insensitive)]
    UKWC,
    #[strum(ascii_case_insensitive)]
    BTHIN,
    #[strum(ascii_case_insensitive)]
    CTHIN,
}

impl Rotor {
    pub fn alpha(&self) -> &'static str {
        match self {
            Rotor::I => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            Rotor::II => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            Rotor::III => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            Rotor::IV => "ESOVPZJAYQUIRHXLNFTGKDCMWB",
            Rotor::V => "VZBRGITYUPSDNHLXAWMJQOFECK",
            Rotor::VI => "JPGVOUMFYQBENHZRDKASXLICTW",
            Rotor::VII => "NZJHGRCXMYSWBOUFAIVLPEKQDT",
            Rotor::VIII => "FKQHTLXOCBJSPDZRAMEWNIUYGV",
            Rotor::BETA => "LEYJVCNIXWPBQMDRTAKZGFUHOS",
            Rotor::GAMMA => "FSOKANUERHMBTIYCWLQPZXVGJD",
        }
    }

    pub fn turnovers(&self) -> &'static str {
        match self {
            Rotor::I => "Q",
            Rotor::II => "E",
            Rotor::III => "V",
            Rotor::IV => "J",
            Rotor::V => "Z",
            Rotor::VI => "ZM",
            Rotor::VII => "ZM",
            Rotor::VIII => "ZM",
            Rotor::BETA => "",
            Rotor::GAMMA => "",
        }
    }
}

impl Reflector {
    fn alpha(&self) -> &'static str {
        match self {
            Reflector::UKWB => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
            Reflector::UKWC => "FVPJIAOYEDRZXWGCTKUQSBNMHL",
            Reflector::BTHIN => "ENKQAUYWJICOPBLMDXZVFTHRGS",
            Reflector::CTHIN => "RDOBJNTKVEHMLFCWZAXGYIPSUQ",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rotors {
    pub reflector: Reflector,
    pub rotors: Vec<Wheel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Wheel {
    Rotor(Rotor),
    Reflector(Reflector),
}

impl Wheel {
    pub fn alpha(&self) -> &'static str {
        match self {
            Wheel::Rotor(x) => x.alpha(),
            Wheel::Reflector(x) => x.alpha(),
        }
    }

    pub fn turnovers(&self) -> &'static str {
        match self {
            Wheel::Rotor(x) => x.turnovers(),
            _ => panic!("There are no turnovers for reflectors"),
        }
    }
}

impl Default for Rotors {
    fn default() -> Self {
        Self {
            reflector: Default::default(),
            rotors: vec![
                Wheel::Rotor(Rotor::I),
                Wheel::Rotor(Rotor::II),
                Wheel::Rotor(Rotor::III),
            ],
        }
    }
}
