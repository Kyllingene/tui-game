use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Difficulty {
    pub food_mul: f32,
    pub food_food_mul: f32,

    pub enemy_mul: f32,
    pub enemy_health_mul: f32,
    pub enemy_damage_mul: f32,
}

impl Difficulty {
    pub const fn food_mul(mut self, food_mul: f32) -> Self { self.food_mul = food_mul; self }
    pub const fn food_food_mul(mut self, food_food_mul: f32) -> Self { self.food_food_mul = food_food_mul; self }

    pub const fn enemy_mul(mut self, enemy_mul: f32) -> Self { self.enemy_mul = enemy_mul; self }
    pub const fn enemy_health_mul(mut self, enemy_health_mul: f32) -> Self { self.enemy_health_mul = enemy_health_mul; self }
    pub const fn enemy_damage_mul(mut self, enemy_damage_mul: f32) -> Self { self.enemy_damage_mul = enemy_damage_mul; self }

    pub const fn easy() -> Self {
        Self::new()
            .food_mul(1.5)
            .food_food_mul(1.2)
            .enemy_mul(0.75)
            .enemy_health_mul(0.8)
            .enemy_damage_mul(0.75)
    }

    pub const fn normal() -> Self {
        Self::new()
    }

    pub const fn hard() -> Self {
        Self::new()
            .food_mul(0.75)
            .food_food_mul(0.8)
            .enemy_mul(1.5)
            .enemy_health_mul(1.5)
            .enemy_damage_mul(2.0)
    }
    
    pub const fn new() -> Self {
        Self {
            food_mul: 1.0,
            food_food_mul: 1.0,

            enemy_mul: 1.0,
            enemy_health_mul: 1.0,
            enemy_damage_mul: 1.0,
        }
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::new()
    }
}

impl Mul for Difficulty {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            food_mul: self.food_mul * other.food_mul,
            food_food_mul: self.food_food_mul * other.food_food_mul,

            enemy_mul: self.enemy_mul * other.enemy_mul,
            enemy_health_mul: self.enemy_health_mul * other.enemy_health_mul,
            enemy_damage_mul: self.enemy_damage_mul * other.enemy_damage_mul,
        }
    }
}

impl std::hash::Hash for Difficulty {
    fn hash<H>(&self, hasher: &mut H) where H: std::hash::Hasher {
        hasher.write_u64((self.food_mul * 100.0) as u64);
        hasher.write_u64((self.food_food_mul * 100.0) as u64);

        hasher.write_u64((self.enemy_mul * 100.0) as u64);
        hasher.write_u64((self.enemy_health_mul * 100.0) as u64);
        hasher.write_u64((self.enemy_damage_mul * 100.0) as u64);
    }
}

pub trait DifficultyMul {
    fn apply(self, multiplier: f32) -> Self;
}

impl DifficultyMul for u32 {
    fn apply(self, multiplier: f32) -> Self {
        (self as f32 * multiplier) as u32
    }
}

