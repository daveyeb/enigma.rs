use crate::rotors::{Rotor, Wheel};
use crate::util::abc;

#[derive(Debug, PartialEq, Clone)]
pub struct Circuit {
    pub code: [usize; 3],
    pub node: Index,
    pub step: bool,

    pub(crate) len: usize,
    pub(crate) turnovers: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Index {
    pub front: usize,
    pub back: isize,
}

// validate

impl Circuit {
    pub fn new(rotors: &Vec<Wheel>, code: [usize; 3]) -> Self {
        let turnovers = rotors
            .iter()
            .map(|r| r.turnovers().to_string())
            .collect::<Vec<_>>();

        let len = rotors.len() + 1; // plus reflector
                                    // stopover
        let back;
        match len > 4 {
            true => back = 8 / 2 - 1,
            false => back = 6 / 2 - 1,
        };

        Circuit {
            code,
            node: Index { front: 0, back },
            step: false,
            len,
            turnovers,
        }
    }

    pub fn reset(&mut self) {
        // stopover
        let back;
        match self.len > 4 {
            true => back = 8 / 2 - 1,
            false => back = 6 / 2 - 1,
        };

        self.node = Index { front: 0, back };
    }


    pub fn signal(&self) -> Box<dyn Fn(u32, [usize; 3]) -> isize + '_> {
        let mut connections = vec!["2", "12", "01", "0"]; // maybe reimplement this
        let mut len = connections.len();

        if self.stopover() % 2 == 1 {
            connections.push("42");
            len = connections.len();
        }

        let at_end = self.node.front > len - 1;
        if at_end {
            connections.reverse();
        }

        let idx = self.node.front % len;
        let mut connection = connections[idx]
            .split_inclusive("")
            .filter(|s| *s != "")
            .collect::<Vec<_>>();

        if at_end {
            connection.reverse()
        }

        match connection.len() - 1 == 1 {
            true => {
                if connection.contains(&"4") {

                    Box::new(move |a: u32, _b: [usize; 3]| {
                        let alpha = abc!()
                            .iter()
                            .position(|c| *c == char::from_u32(a).unwrap_or('\0') as u32)
                            .unwrap();
                        alpha as isize
                    })
                } else {

                    Box::new(move |a: u32, b: [usize; 3]| {
                        let alpha = abc!()
                            .iter()
                            .position(|c| *c == char::from_u32(a).unwrap_or('\0') as u32)
                            .unwrap();

                        let wheels = [
                            connection[0].parse::<usize>().unwrap(),
                            connection[1].parse::<usize>().unwrap(),
                        ];

                        let mut sig =
                            alpha as isize + (b[wheels[0]] as isize - b[wheels[1]] as isize);
                        sig = sig % 26;

                        match sig < 0 {
                            true => sig + 26,
                            false => sig,
                        }
                    })
                }
            }
            false => {
                let node_parity;
                match len > 4 && at_end {
                    true => node_parity = idx % 2 == 0,
                    false => node_parity = idx % 2 == 1,
                };

                if node_parity {
                    Box::new(move |a: u32, b: [usize; 3]| {
                        let alpha = abc!()
                            .iter()
                            .position(|c| *c == char::from_u32(a).unwrap_or('\0') as u32)
                            .unwrap();


                        let wheel = connections[self.node.front % len].parse::<usize>().unwrap();
                        let mut sig = alpha as isize - b[wheel] as isize;
                        sig = sig % 26;

                        match sig < 0 {
                            true => sig + 26,
                            false => sig,
                        }
                    })
                } else {
                    Box::new(move |a: u32, b: [usize; 3]| {
                        let alpha = abc!()
                            .iter()
                            .position(|c| *c == char::from_u32(a).unwrap_or('\0') as u32)
                            .unwrap();

                        let wheel = connections[self.node.front % len].parse::<usize>().unwrap();

                        let mut sig = alpha as isize + b[wheel] as isize;
                        sig = sig % 26;

                        match sig < 0 {
                            true => sig + 26,
                            false => sig,
                        }
                    })
                }
            }
        }
    }

    pub fn pathway(&self) -> u8 {
        match self.len > 4 {
            true => 8,
            false => 6,
        }
    }

    pub fn stopover(&self) -> u8 {
        self.pathway() / 2 - 1
    }

    fn node_step(&mut self) -> bool {
        if self.node.front <= self.pathway().into() {
            if !(self.node.front < self.len) {
                self.node.back -= 1;
            }
            self.node.front += 1;
        }

        self.node.front <= self.pathway().into()
    }

    pub fn accrue(&mut self) {
        let etw = ('A'..='Z').into_iter().collect::<Vec<_>>();

        let mut turnover = self.turnovers[1].split_inclusive("").collect::<Vec<_>>();
        let mut index = self.code[1] as usize;

        if !turnover.contains(&etw[index].to_string().as_str()) {
            self.step = false;
        }

        if turnover.contains(&etw[index].to_string().as_str()) && !self.step {
            self.code[2] = (self.code[2] + 1) % 26;
            self.code[1] = (self.code[1] + 1) % 26;
            self.code[0] = (self.code[0] + 1) % 26;

            self.step = true;
            return;
        }

        turnover = self.turnovers[0].split_inclusive("").collect::<Vec<_>>();
        index = self.code[2] as usize;

        if turnover.contains(&etw[index].to_string().as_str()) {
            self.code[2] = (self.code[2] + 1) % 26;
            self.code[1] = (self.code[1] + 1) % 26;

            return;
        }

        self.code[2] = (self.code[2] + 1) % 26;
    }

    pub fn scramble(&mut self, ch: &char, rotors: &Vec<Wheel>) -> char {
        let mut pch = ch.to_owned();

        loop {
            if self.node.front < self.len {
                pch = rotors[self.node.front].alpha().chars().collect::<Vec<_>>()
                    [self.signal()(pch as u32, self.code) as usize];
            } else {
                let alpha = abc!();

                let tch =
                    char::from_u32(alpha[self.signal()(pch as u32, self.code) as usize]).unwrap();

                pch = char::from_u32(
                    alpha[rotors[self.node.back as usize]
                        .alpha()
                        .chars()
                        .position(|c| c == tch)
                        .unwrap()],
                )
                .unwrap();
            }

            if !self.node_step() {
                break;
            }
        }

        pch
    }
}

impl Default for Circuit {
    fn default() -> Self {
        let rotors = vec![Rotor::I, Rotor::II, Rotor::III];
        let turnovers = rotors
            .iter()
            .map(|r| r.turnovers().to_string())
            .collect::<Vec<_>>();

        Self {
            code: [0, 0, 0],
            node: Index {
                front: 0,
                back: 6 / 2 - 1,
            },
            step: false,
            len: 4,
            turnovers,
        }
    }
}

impl Default for Index {
    fn default() -> Self {
        Self {
            front: Default::default(),
            back: Default::default(),
        }
    }
}

#[test]
pub fn test_node_stepping() {
    let mut circuit = Circuit::default();

    loop {
        if !circuit.node_step() {
            break;
        }
    }
}
