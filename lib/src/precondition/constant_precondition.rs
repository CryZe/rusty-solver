use PreCondition;

pub struct ConstantPreCondition(pub f32);

impl PreCondition for ConstantPreCondition {
    fn precondition(&self, _: (usize, usize)) -> f32 {
        let &ConstantPreCondition(value) = self;
        value
    }
}
