use object::method;


pub trait Operators:
method::Add +
method::BitwiseAnd +
method::DivMod +
method::FloorDivision +
method::LeftShift +
method::Modulus +
method::Multiply+
method::MatrixMultiply +
method::BitwiseOr +
method::Pow + // Not exactly a binary op
method::RightShift +
method::Subtract +
method::TrueDivision +
method::XOr +
method::ReflectedAdd +
method::ReflectedBitwiseAnd +
method::ReflectedDivMod +
method::ReflectedFloorDivision +
method::ReflectedLeftShift +
method::ReflectedModulus +
method::ReflectedMultiply+
method::ReflectedMatrixMultiply +
method::ReflectedBitwiseOr +
method::ReflectedPow + // Not exactly a binary op
method::ReflectedRightShift +
method::ReflectedSubtract +
method::ReflectedTrueDivision +
method::ReflectedXOr +
method::InPlaceAdd +
method::InPlaceBitwiseAnd +
method::InPlaceDivMod +
method::InPlaceFloorDivision +
method::InPlaceLeftShift +
method::InPlaceModulus +
method::InPlaceMultiply+
method::InPlaceMatrixMultiply +
method::InPlaceBitwiseOr +
method::InPlacePow + // Not exactly a binary op
method::InPlaceRightShift +
method::InPlaceSubtract +
method::InPlaceTrueDivision +
method::InPlaceXOr {}
