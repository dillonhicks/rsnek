
#[macro_export]
macro_rules! foreach_builtin {
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident) => (
        unary_op_foreach!($obj, $rt, $op, $lhs)
    );
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $rhs:ident) => (
        binary_op_foreach!($obj, $rt, $op, $lhs, $rhs)
    );
    ($op:expr, $rt:expr, $lhs:expr, $infix:ident, $rhs:expr, $obj:expr, $rt:expr) => (
        ternary_op_foreach!($obj, $rt, $op, $lhs, $infix, $rhs)
    )
}

macro_rules! unary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident) => {

        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt),
            &Builtin::Integer(ref $lhs) => $lhs.$op($rt),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt),
            &Builtin::String(ref $lhs) => $lhs.$op($rt),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt),
            &Builtin::List(ref $lhs) => $lhs.$op($rt),
            &Builtin::Set(ref $lhs) => $lhs.$op($rt),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt),
            &Builtin::Complex(ref $lhs) => $lhs.$op($rt),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt),
            _ => Err(Error::not_implemented())
        }
    };
}

macro_rules! binary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $rhs:ident) => {

        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Integer(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::String(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Set(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Complex(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt, $rhs),
            _ => Err(Error::not_implemented())
        }
    };
}

macro_rules! ternary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Integer(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::String(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Set(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Complex(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            _ => Err(Error::not_implemented())
        }
    };
}

#[macro_export]
macro_rules! native_foreach_builtin {
    ($obj:expr, $op:ident, $lhs:ident) => (
        native_unary_op_foreach!($obj, $op, $lhs)
    );
    ($obj:expr, $op:ident, $lhs:ident, $rhs:ident) => (
        native_binary_op_foreach!($obj, $op, $lhs, $rhs)
    );
    ($op:expr, $lhs:expr, $infix:ident, $rhs:expr, $obj:expr) => (
        native_ternary_op_foreach!($obj, $op, $lhs, $infix, $rhs)
    )

}

macro_rules! native_unary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident) => {

        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op(),
            &Builtin::Integer(ref $lhs) => $lhs.$op(),
            &Builtin::Float(ref $lhs) => $lhs.$op(),
            &Builtin::String(ref $lhs) => $lhs.$op(),
            &Builtin::Tuple(ref $lhs) => $lhs.$op(),
            &Builtin::List(ref $lhs) => $lhs.$op(),
            &Builtin::Set(ref $lhs) => $lhs.$op(),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op(),
            &Builtin::Complex(ref $lhs) => $lhs.$op(),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op(),
            _ => panic!("Unhandled native case in macro!")
        }
    };
}

macro_rules! native_binary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $rhs:ident) => {

        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Integer(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rhs),
            &Builtin::String(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Set(ref $lhs) => $lhs.$op($rhs),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Complex(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rhs),
            _ => panic!("Unhandled native case in macro!")
        }
    };
}

macro_rules! native_ternary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Boolean(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Integer(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::String(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Set(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Complex(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Dictionary(ref $lhs) => $lhs.$op($mid, $rhs),
            _ => panic!("Unhandled native case in macro!")
        }
    };
}
