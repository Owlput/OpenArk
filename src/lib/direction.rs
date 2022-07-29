use std::{f32::consts::PI, ops::AddAssign};

#[derive(Clone, PartialEq)]
pub enum Direction {
    L,
    R,
    F,
    B,
    FL,
    FR,
    BL,
    BR,
    O,
}
//-A pretty dumb way to determine the direction
// Hoping for a better solution
//一个判断方向的蠢办法，看看有没有更聪明的
impl Direction {
    pub fn to_angle(self) -> f32 {
        match self {
            Direction::L => PI / 2.,
            Direction::R => -PI / 2.,
            Direction::F => 0.,
            Direction::B => PI,
            Direction::FL => PI / 4.,
            Direction::FR => -PI / 4.,
            Direction::BL => 3. * PI / 4.,
            Direction::BR => -3. * PI / 4.,
            Direction::O => 0.,
        }
    }
}
impl AddAssign for Direction {
    //Operator overload
    //操作符重载
    //Some cases are not covered here because we simply won't have to deal with them
    //so we can write less unnecessary code.
    //没有枚举所有可能，因为有很多情况我们不会遇到，让代码少一点
    //If it doesn't make sense, then see how we handle keyboard input below
    //如果还不怎么理解可以看看下面是怎么处理键盘输入的
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Direction::O => match rhs {
                Direction::F => *self = Direction::F,
                Direction::B => *self = Direction::B,
                Direction::L => *self = Direction::L,
                Direction::R => *self = Direction::R,
                _ => *self = Direction::O,
            },
            Direction::F => match rhs {
                Direction::B => *self = Direction::O,
                Direction::L => *self = Direction::FL,
                Direction::R => *self = Direction::FR,
                _ => *self = Direction::O,
            },
            Direction::B => match rhs {
                Direction::L => *self = Direction::BL,
                Direction::R => *self = Direction::BR,
                _ => *self = Direction::O,
            },

            Direction::FL => match rhs {
                Direction::R => *self = Direction::F,
                _ => *self = Direction::O,
            },
            Direction::BL => *self = Direction::B,
            Direction::L => *self = Direction::O,
            _ => *self = Direction::O,
        }
        ()
    }
}