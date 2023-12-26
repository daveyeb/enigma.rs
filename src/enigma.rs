use std::{collections::HashMap, iter::zip, str::FromStr};

use crate::{
    circuit::{Circuit, Index},
    rotors::{Reflector, Rotor, Rotors, Wheel},
    util::abc,
};

pub(crate) type Stecker = HashMap<u32, u32>;
#[derive(Debug, PartialEq)]
pub struct Enigma {
    pub wheels: Rotors,
    circuit: Circuit,
    pub plugboard: Stecker,
}

impl Enigma {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_wheels(rotors: &[&str], reflector: &str) -> Self {
        let mut _rotors = Rotors::default();
        let plugboard: Stecker = zip(abc!(), abc!()).into_iter().clone().collect();

        if rotors.len() >= 5 || rotors.len() <= 2 {
            panic!("{:?} Too many rotors, should be max 4 or min 3", rotors);
        }

        let len = rotors.len();
        _rotors.rotors.clear();

        if rotors.len() == 4 {}

        for x in rotors {
            let mut beta_count = 2;
            let mut gamma_count = 2;
            match Rotor::from_str(x) {
                Ok(r) => {
                    let valid = vec![
                        "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "beta", "gamma",
                    ];

                    if !valid.contains(&x) {
                        panic!(
                            "{:?} Rotor not valid, cant choose this rotor. Acceptable rotors: {:?}",
                            x, valid
                        );
                    }

                    match r {
                        Rotor::BETA => beta_count -= 1,
                        Rotor::GAMMA => gamma_count -= 1,
                        _ => (),
                    }

                    if beta_count <= 0 || gamma_count <= 0 {
                        panic!(
                            "{:?} Beta or Gamma rotors cant be used more than twice",
                            rotors
                        );
                    }

                    _rotors.rotors.push(Wheel::Rotor(r))
                }
                Err(_) => panic!("{:?} Rotor doesn't exist, try an existing one", x),
            }
        }

        match Reflector::from_str(reflector) {
            Ok(x) => {
                let mut valid = vec!["ukwb", "ukwc"];
                if len == 3 {
                    valid = vec!["ukwb", "ukwc", "bthin", "cthin"]
                }

                if !valid.contains(&reflector) {
                    panic!("{:?} Reflector not valid, cant choose this reflector for a {} numbered rotors. Acceptable reflectors: {:?}", rotors, len, valid);
                }

                _rotors.reflector = x
            }
            Err(_) => panic!(
                "{:?} Reflector doesn't exist, try an existing one",
                reflector
            ),
        }

        Enigma::from(&mut _rotors, None, None)
    }

    pub fn from_config(
        rotors: &[&str],
        reflector: &str,
        code_settings: Option<[char; 3]>,
        plug_settings: Option<&[(char, char)]>,
    ) -> Self {
        let mut _rotors = Rotors::default();
        let len = rotors.len();

        if rotors.len() >= 5 || rotors.len() <= 2 {
            panic!("{:?} Too many rotors, should be max 4 or min 3", rotors);
        }

        _rotors.rotors.clear();
        for x in rotors {
            match Rotor::from_str(x) {
                Ok(r) => {
                    let valid = vec![
                        "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "beta", "gamma",
                    ];

                    if !valid.contains(&x) {
                        panic!(
                            "{:?} Rotor not valid, cant choose this rotor. Acceptable rotors: {:?}",
                            x, valid
                        );
                    }

                    _rotors.rotors.push(Wheel::Rotor(r))
                }
                Err(_) => panic!("{:?} Rotor doesn't exist, try an existing one", x),
            }
        }

        match Reflector::from_str(reflector) {
            Ok(x) => {
                let mut valid = vec!["ukwb", "ukwc"];
                if len == 4 {
                    valid = vec!["ukwb", "ukwc", "bthin", "cthin"]
                }

                if !valid.contains(&reflector) {
                    panic!("{:?} Reflector not valid, cant choose this reflector for a {} numbered rotors. Acceptable reflectors: {:?}", reflector, len, valid);
                }

                _rotors.reflector = x
            }
            Err(_) => panic!(
                "{:?} Reflector doesn't exist, try an existing one",
                reflector
            ),
        }

        Enigma::from(&mut _rotors, code_settings, plug_settings)
    }

    fn from(
        config_wheel: &mut Rotors,
        code_settings: Option<[char; 3]>,
        plug_settings: Option<&[(char, char)]>,
    ) -> Self {
        let mut code = [0, 0, 0];
        let mut plugboard: Stecker = zip(abc!(), abc!()).into_iter().clone().collect();

        if let Some(code_settings) = code_settings {
            let mut index = 0;
            for c in code_settings {
                let alpha = abc!().iter().position(|ch| *ch == c as u32).unwrap();

                code[index] = alpha;
                index += 1;
            }
        }

        let _ = &config_wheel.rotors.reverse();
        let circuit = Circuit::new(&config_wheel.rotors, code);

        config_wheel
            .rotors
            .push(Wheel::Reflector(config_wheel.reflector.clone()));

        if let Some(plug_settings) = plug_settings {
            for i in plug_settings {
                *plugboard.get_mut(&(i.0 as u32)).unwrap() = i.1 as u32;
                *plugboard.get_mut(&(i.1 as u32)).unwrap() = i.0 as u32;
            }
        }

        // assert_eq!(plugboard, zip(abc!(), abc!()).into_iter().clone().collect());

        Enigma {
            wheels: config_wheel.clone(),
            circuit,
            plugboard,
        }
    }

    pub fn encode(&mut self, plaintext: &str) -> String {
        let rotors = self.wheels.rotors.clone();
        let etw = abc!();

        let ciphertext = plaintext.chars().map(|mut ch| {
            // elimate unwanted chars
            if ch.is_whitespace() {
                return ch;
            }

            self.circuit.accrue();

            let ch_pos = &(etw.iter().position(|a| *a == ch as u32).unwrap() as u32);
            ch = char::from_u32(self.plugboard[&(ch_pos + 65)] as u32)
                .unwrap()
                .clone();

            ch = self.circuit.scramble(&ch, &rotors);
            ch = char::from_u32(
                self.plugboard
                    [&((self.circuit.signal()(ch as u32, self.circuit.code) + 65) as u32)],
            )
            .unwrap();

            self.circuit.reset();

            ch
        });

        ciphertext.into_iter().collect()
    }

    pub fn decode(&mut self, ciphertext: &str) -> String {
        self.encode(ciphertext)
    }
}

impl Default for Enigma {
    fn default() -> Self {
        let plugboard: Stecker = zip(abc!(), abc!()).into_iter().clone().collect();
        Self {
            wheels: Default::default(),
            circuit: Default::default(),
            plugboard,
        }
    }
}

#[test]
pub fn create_enigma() {
    let plugboard: Stecker = zip(abc!(), abc!()).into_iter().clone().collect();

    let enigma = Enigma {
        wheels: Rotors {
            reflector: Reflector::UKWB,
            rotors: vec![
                Wheel::Rotor(Rotor::I),
                Wheel::Rotor(Rotor::II),
                Wheel::Rotor(Rotor::III),
            ],
        },
        circuit: Circuit {
            code: [0, 0, 0],
            node: Index {
                front: 0,
                back: 6 / 2 - 1,
            },
            step: false,
            len: 4,
            turnovers: vec!["Q".to_string(), "E".to_string(), "V".to_string()],
        },
        plugboard,
    };

    assert_eq!(enigma, Enigma::new());
}

#[test]
pub fn create_enigma_w_config() {
    let plugboard: Stecker = zip(abc!(), abc!()).into_iter().clone().collect();

    let enigma = Enigma {
        wheels: Rotors {
            reflector: Reflector::UKWB,
            rotors: vec![
                Wheel::Rotor(Rotor::I),
                Wheel::Rotor(Rotor::II),
                Wheel::Rotor(Rotor::III),
            ],
        },
        circuit: Circuit {
            code: [0, 0, 0],
            node: Index {
                front: 0,
                back: 6 / 2 - 1,
            },
            step: false,
            len: 4,
            turnovers: vec!["Q".to_string(), "E".to_string(), "V".to_string()],
        },
        plugboard,
    };

    assert_eq!(
        enigma,
        Enigma::from_config(&["i", "ii", "iii"], "ukwb", None, None)
    );
}

#[test]
pub fn create_enigma_w_config_w_m4m3combos() {
    let plugboard = [('X', 'S'), ('F', 'V')];
    let code = Some(['Q', 'E', 'V']);

    let mut enigma = Enigma::from_config(&["i", "ii", "iii"], "ukwc", None, Some(&plugboard));

    println!("{}", enigma.encode("WTPXFMWXLIW"))
}
