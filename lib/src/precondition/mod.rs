mod constant_precondition;

pub use self::constant_precondition::ConstantPreCondition;

pub trait PreCondition {
    fn precondition(&self, coord: (usize, usize)) -> f32;
}
