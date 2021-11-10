use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub struct powers;

impl powers {
    pub fn init() -> Result<Self, String> {
        Ok(powers {})
    }

    pub fn pickup_power() -> Option<PowerUps> {
        return Some(rand::random());
    }

    pub fn handler(power: Option<PowerUps>) -> bool {
        match power {
            Some(PowerUps::SpeedBoost) => {
                return speed_boost();
            }
            Some(PowerUps::ScoreMultiplier) => {
                return score_mul();
            }
            Some(PowerUps::BouncyShoes) => {
                return bouncy_shoes();
            }
            Some(PowerUps::LowerGravity) => {
                return lower_gravity();
            }
            Some(PowerUps::Shield) => {
                return shield();
            }
            _ => return false,
        }
    }
}

pub enum PowerUps {
    SpeedBoost,
    ScoreMultiplier,
    BouncyShoes,
    LowerGravity,
    Shield,
}

pub fn speed_boost() -> bool {
    //Every tick active, Apply faster static increase to velocity or acceleration
    return false;
}

pub fn score_mul() -> bool {
    //Every tick active, take however many points obtained and apply a multiplier
    return false;
}

pub fn bouncy_shoes() -> bool {
    //Every tick active, if player lands on ground or obstacle, start another jump to a lesser height if possible
    return false;
}

pub fn lower_gravity() -> bool {
    //Every tick active, make the gravity force lower so the player is more "floaty"
    return false;
}

pub fn shield() -> bool {
    //Every tick active, player cannot crash due to bad flip or hitting an obstacle
    return false;
}

impl Distribution<PowerUps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUps {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=4) {
            // rand 0.8
            0 => PowerUps::SpeedBoost,
            1 => PowerUps::ScoreMultiplier,
            2 => PowerUps::BouncyShoes,
            3 => PowerUps::LowerGravity,
            _ => PowerUps::Shield,
        }
    }
}
