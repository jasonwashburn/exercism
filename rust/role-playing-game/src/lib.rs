pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut new_player = Player {
                health: 100,
                mana: None,
                level: self.level,
            };
            if self.level >= 10 {
                new_player.mana = Some(100);
            }
            Some(new_player)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    0
                } else {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                }
            }
            None => {
                if self.health > mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                0
            }
        }
    }
}
