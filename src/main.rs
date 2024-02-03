use std::str::FromStr;

enum AttackType {
    Physical,
    Magic,
    Flexible,
    Assault,
}

impl FromStr for AttackType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "p" | "P" => Ok(Self::Physical),
            "m" | "M" => Ok(Self::Magic),
            "x" | "X" => Ok(Self::Flexible),
            "a" | "A" => Ok(Self::Assault),
            _ => Err(()),
        };
        result
    }
}

struct Card {
    attack_value: u8,
    attack_type: AttackType,
    physical_defense: u8,
    magical_defense: u8,
}

impl Card {
    fn new(card_in: &str) -> Option<Self> {
        let chars: Vec<char> = card_in.chars().collect();
        if chars.len() != 4 {
            return None;
        }
        let attack_value = match u8::from_str_radix(chars[0].to_string().as_str(), 16) {
            Ok(valid_attack_value) => valid_attack_value,
            Err(_) => return None,
        };
        let attack_type = match AttackType::from_str(chars[1].to_string().as_str()) {
            Ok(valid_attack_type) => valid_attack_type,
            Err(_) => return None,
        };
        let physical_defense = match u8::from_str_radix(chars[2].to_string().as_str(), 16) {
            Ok(valid_physical_defense_value) => valid_physical_defense_value,
            Err(_) => return None,
        };
        let magical_defense = match u8::from_str_radix(chars[3].to_string().as_str(), 16) {
            Ok(valid_magical_defense_value) => valid_magical_defense_value,
            Err(_) => return None,
        };
        Some(Self {
            attack_value,
            attack_type,
            physical_defense,
            magical_defense,
        })
    }

    fn battle_calc(&self, other: &Card) -> f64 {
        match self.attack_type {
            AttackType::Physical => {
                let attack_power = self.attack_value;
                let defender_power = other.physical_defense;
                probability_of_win(attack_power, defender_power)
            }
            AttackType::Magic => {
                let attack_power = self.attack_value;
                let defender_power = other.magical_defense;
                probability_of_win(attack_power, defender_power)
            }
            AttackType::Flexible => {
                let attack_power = self.attack_value;
                let defender_power = if other.physical_defense > other.magical_defense {
                    other.magical_defense
                } else {
                    other.physical_defense
                };
                probability_of_win(attack_power, defender_power)
            }
            AttackType::Assault => {
                let mut attack_list = [
                    self.attack_value,
                    self.physical_defense,
                    self.magical_defense,
                ];
                attack_list.sort();
                let attack_power = attack_list[2];
                let mut defender_list = [
                    other.attack_value,
                    other.physical_defense,
                    other.magical_defense,
                ];
                defender_list.sort();
                let defender_power = defender_list[0];
                probability_of_win(attack_power, defender_power)
            }
        }
    }
}

fn probability_of_win(a: u8, d: u8) -> f64 {
    let mut wins: usize = 0;
    let mut losses: usize = 0;

    let attacker_upper_range = 16 * a..16 * (a + 1);
    let defender_upper_range = 16 * d..16 * (d + 1);

    for attack_upper in attacker_upper_range {
        for defense_upper in defender_upper_range.clone() {
            for attack_roll in 0..=attack_upper {
                if attack_roll < defense_upper {
                    wins += attack_roll as usize;
                    losses += defense_upper as usize - attack_roll as usize + 1;
                } else {
                    wins += defense_upper as usize;
                }
            }
        }
    }

    wins as f64 / (wins as f64 + losses as f64)
}

fn main() {
    let attacker = std::env::args().nth(1).expect("Attacker missing");
    let defender = std::env::args().nth(2).expect("Defender missing");

    let attacker_card = Card::new(&attacker).expect("Invalid Attacker");
    let defender_card = Card::new(&defender).expect("Invalid Defender");

    println!(
        "Odds of win: {:.2}%",
        attacker_card.battle_calc(&defender_card) * 100.0
    )
}
