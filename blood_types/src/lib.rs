use std::cmp::{Ord, Ordering};
use std::str::FromStr;
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh_char = s.chars().last().unwrap();
        let rh_factor = match rh_char {
            '+' => RhFactor::Positive,
            '-' => RhFactor::Negative,
            _ => return Err(()),
        };

        let antigen_part = &s[0..s.len()-1];
        let antigen = match antigen_part {
            "A" => Antigen::A,
            "AB" => Antigen::AB,
            "B" => Antigen::B,
            "O" => Antigen::O,
            _ => return Err(()),
        };

        Ok(BloodType { antigen, rh_factor })
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }

        if other.antigen == Antigen::O {
            return true;
        }

        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let mut compatible_antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        let compatible_rh = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        if self.antigen == Antigen::AB {
            compatible_antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for rh in compatible_rh.iter() {
            for antigen in compatible_antigens.iter() {
                result.push(BloodType {
                    rh_factor: rh.clone(),
                    antigen: antigen.clone(),
                })
            }
        }

        result
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut result = Vec::new();
        let mut compatible_antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };

        let compatible_rh = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };

        if self.antigen == Antigen::O {
            compatible_antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for rh in compatible_rh.iter() {
            for antigen in compatible_antigens.iter() {
                result.push(BloodType {
                    rh_factor: rh.clone(),
                    antigen: antigen.clone(),
                })
            }
        }

        result
    }
}