# enigma.rs

A terminal based program that mimicks the operations of an Enigma M3/M4 cipher machine developed by Germans during WWII.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- A more robust encryption machine, performs much better than daveyeb/node-engima.
- M3 and M4 encryption capabilities.

## Installation

```sh
$ git clone https://github.com/daveyeb/enigma.rs.git
$ cd enigma.rs
$ cargo build
```
## Usage

```rust 
use crate::enigma::Enigma;

fn main() {
    let plugboard = [('X', 'S'), ('F', 'V')];
    let mut enigma = Enigma::from_config(&["i", "ii", "iii"], "ukwc", None, Some(&plugboard));

    println!("{}", enigma.encode("EPAUK PEDJERBYLE HIIXJ MQVLJDNCYF BYLUZ QMVGOVQXXC JFGOF CWJEEMPWUZ GFWLF ZMUBHZRWXQ GDAWH KCIAUIJSWO CGSRH NRFKQPDHLT QYLBE FRGMLQEJMB NOUYD JBYITCVNMM KGGIP VYMVGSCFHN"))
}

```

## Contributing

If you encounter any issues or wish to contribute improvements, you're welcome to clone the repository, make modifications, conduct testing, and submit pull requests.

## License 

This project is licensed under the MIT - see the [LICENSE](/LICENSE) file for details.
