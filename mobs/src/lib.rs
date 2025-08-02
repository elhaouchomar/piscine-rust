use std::collections::{HashMap, HashSet};

pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn new(name: String, boss: Boss) -> Self {
        Self {
            name,
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        let member = Member::new(Role::Associate, age);
        self.members.insert(name.to_string(), member);
    }

    fn calculate_power(&self) -> u32 {
        self.members.values().map(|member| member.role.power()).sum()
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power = self.calculate_power();
        let other_power = other.calculate_power();

        let loser = if self_power > other_power {
            other
        } else {
            // In case of draw, attacker loses
            self
        };

        // Remove youngest member(s) from the losing mob
        if !loser.members.is_empty() {
            let min_age = loser.members.values().map(|m| m.age).min().unwrap();
            let youngest_names: Vec<String> = loser
                .members
                .iter()
                .filter(|(_, member)| member.age == min_age)
                .map(|(name, _)| name.clone())
                .collect();

            for name in youngest_names {
                loser.members.remove(&name);
            }

            // If loser has no members left, transfer cities and wealth
            if loser.members.is_empty() {
                let winner = if self_power > other_power { self } else { other };
                winner.cities.extend(loser.cities.drain());
                winner.wealth += loser.wealth;
                loser.wealth = 0;
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen = amount.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city_name: String) {
        let city_already_owned = other_mobs
            .iter()
            .any(|mob| mob.cities.contains(&city_name));

        if !city_already_owned {
            self.cities.insert(city_name);
        }
    }
}
