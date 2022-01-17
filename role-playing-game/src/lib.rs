// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: Player::define_mana(self.level),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health - cmp::min(mana_cost, self.health);
                return 0;
            },
            Some(value) => {
                if value > mana_cost {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    return mana_cost * 2;
                } else {
                    return 0;
                }
            }
        }
    }

    pub fn define_mana(level: u32) -> Option<u32> {
        if level >= 10 {
            return Some(100);
        }

        None
    }
}
