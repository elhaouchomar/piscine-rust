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
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
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

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            other => other,
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh_factor = if s.ends_with('+') {
            RhFactor::Positive
        } else if s.ends_with('-') {
            RhFactor::Negative
        } else {
            return Err(());
        };

        let antigen_str = &s[..s.len() - 1];
        let antigen = antigen_str.parse::<Antigen>()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_compatible = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            (RhFactor::Negative, RhFactor::Positive) => false,
        };

        if !rh_compatible {
            return false;
        }

        match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            (Antigen::O, _) => false,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::A, _) => false,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::B, _) => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        all_blood_types
            .into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        all_blood_types
            .into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }
}