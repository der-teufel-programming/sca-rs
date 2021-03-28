use std::str::FromStr;

pub struct Rule {
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
