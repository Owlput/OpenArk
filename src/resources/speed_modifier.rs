pub struct SpeedModifier {
    speed: f64,
}

impl SpeedModifier {
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