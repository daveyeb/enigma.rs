use crate::enigma::Enigma;

mod rotors;
mod circuit;
mod enigma;
mod util;

fn main() {
    let plugboard = [('X', 'S'), ('F', 'V')];
    let mut enigma = Enigma::from_config(&["i", "ii", "iii"], "ukwc", None, Some(&plugboard));

    println!("{}", enigma.encode("EPAUK PEDJERBYLE HIIXJ MQVLJDNCYF BYLUZ QMVGOVQXXC JFGOF CWJEEMPWUZ GFWLF ZMUBHZRWXQ GDAWH KCIAUIJSWO CGSRH NRFKQPDHLT QYLBE FRGMLQEJMB NOUYD JBYITCVNMM KGGIP VYMVGSCFHN"))
}
