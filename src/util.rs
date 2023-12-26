macro_rules! abc {
    () => {
        ('A'..='Z')
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x)
            .map(|x| x as u32)
            .collect::<Vec<u32>>()
    };
}

macro_rules! abc_index {
    ($s:expr) => {
        ('A'..='Z')
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x as u32)
            .position(|x| x == ($s as u32))
    };
}

pub(crate) use abc;
pub(crate) use abc_index;
