use rand::Rng;

pub enum PowerUps {
    SpeedBoost,
    ScoreMultiplier,
    BouncyShoes,
    LowerGravity,
    Shield,
}

pub fn pickup_power() -> Option<PowerUps> {
    let mut rng = rand::thread_rng();

    //There is some better way to use rand to explicilty choose one but its annoying
    let check = rng.gen::<f64>();
    let val = 1.0 / 5.0; //1.0 / number of possible powers

    if check < val {
        return PowerUps::SpeedBoost;
    } else if check < val * 2 {
        return PowerUps::ScoreMultiplier;
    } else if check < val * 3 {
        return PowerUps::BouncyShoes;
    } else if check < val * 4 {
        return PowerUps::LowerGravity;
    } else if check < val * 5 {
        return PowerUps::Shield;
    }
    return PowerUps::SpeedBoost;
}

pub fn handler(power: Option<PowerUps>) -> bool {
    match power {
        PowerUps::SpeedBoost => {
            return speed_boost();
        }
        PowerUps::ScoreMultiplier => {
            return score_mul();
        }
        PowerUps::BouncyShoes => {
            return bouncy_shoes();
        }
        PowerUps::LowerGravity => {
            return lower_gravity();
        }
        PowerUps::Shield => {
            return shield();
        }
        _ => return false,
    }
}

pub fn speed_boost() -> bool {
    //Every tick active, Apply faster static increase to velocity or acceleration
    false;
}

pub fn score_mul() -> bool {
    //Every tick active, take however many points obtained and apply a multiplier
    false;
}

pub fn bouncy_shoes() -> bool {
    //Every tick active, if player lands on ground or obstacle, start another jump to a lesser height if possible
    false;
}

pub fn lower_gravity() -> bool {
    //Every tick active, make the gravity force lower so the player is more "floaty"
    false;
}

pub fn shield() -> bool {
    //Every tick active, player cannot crash due to bad flip or hitting an obstacle
    false;
}
