/// A multiplier for game speed. 1.0 is normal speed.  
/// Handle with care.
pub struct GameSpeedModifier {
    speed: f64,
}

impl GameSpeedModifier {
    pub fn new(multiplier:f64) -> Self {
        Self { speed: multiplier }
    }
    pub fn change_to(&mut self,speed:f64){
        self.speed = speed
    }
    pub fn get(&self)->f64{
        self.speed
    }
}