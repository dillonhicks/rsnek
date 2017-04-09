use object::method;


/// Cohesive Number trait for convenience
pub trait Number:
    method::BooleanCast +
    method::IntegerCast +
    method::FloatCast +
    method::ComplexCast +
    method::Rounding +
    method::Index +
    method::NegateValue +
    method::AbsValue +
    method::PositiveValue +
    method::InvertValue {}
