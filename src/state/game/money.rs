use bevy::prelude::*;

use std::fmt::Formatter;
use std::ops::AddAssign;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Money(u128);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Money {
    pub fn for_killing_enemy() -> Self {
        Self(10)
    }
    pub fn can_buy(&self, cost: &TowerCost) -> bool {
        self.0 >= cost.0
    }
    pub fn buy(&mut self, cost: &TowerCost) {
        self.0 -= cost.0;
    }
}

impl AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.0 += rhs.0
    }
}

#[derive(Component)]
pub struct TowerCost(u128);

impl std::fmt::Display for TowerCost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for TowerCost {
    fn default() -> Self {
        TowerCost(10)
    }
}
