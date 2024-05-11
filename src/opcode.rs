pub type Value = i32;


#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Constant(Value),

    Return,
}
