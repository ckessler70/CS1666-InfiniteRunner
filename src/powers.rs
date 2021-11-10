use rand::Rng;

pub struct powers;

impl powers {
    pub fn init() -> Result<Self, String> {
        Ok(powers {})
    }

    pub fn pickup_power() -> Option<PowerUps> {
        let mut rng = rand::thread_rng();

        //There is some better way to use rand to explicilty choose one but its annoying
        let check = rng.gen::<f64>();
        let val = 1.0 / 5.0; //1.0 / number of possible powers

        if check < val {
            return Some(PowerUps::SpeedBoost);
        } else if check < val * 2.0 {
            return Some(PowerUps::ScoreMultiplier);
        } else if check < val * 3.0 {
            return Some(PowerUps::BouncyShoes);
        } else if check < val * 4.0 {
            return Some(PowerUps::LowerGravity);
        } else if check < val * 5.0 {
            return Some(PowerUps::Shield);
        }
        return Some(PowerUps::SpeedBoost);
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
