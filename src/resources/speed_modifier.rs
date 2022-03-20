pub struct SpeedModifier {
    speed: f32,
}

impl SpeedModifier {
    pub fn new(multiplier:f32) -> Self {
        Self { speed: multiplier }
    }
    pub fn change_to(&mut self,speed:f32){
        self.speed = speed
    }
    pub fn get(&self)->f32{
        self.speed
    }
}