use bevy::prelude::*;

use std::fmt::Formatter;
use std::ops::AddAssign;

#[derive(Component, Debug, Default, Deref, DerefMut)]
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
}

impl AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.0 += rhs.0
    }
}
