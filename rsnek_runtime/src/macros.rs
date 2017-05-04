
macro_rules! try_cast {
    ($out:ident, $objref:expr, $builtin:path) => (
        let boxed: &Box<Builtin> = $objref.0.borrow();
        match boxed.deref() {
            &$builtin(ref obj) => $out = &obj,
            _ => panic!("Not expected type")
        }
    )
}


macro_rules! foreach_builtin {
    ($sel:expr, $rt:expr, $function:ident, $receiver:ident) => (
        unary_op_foreach!($sel, $rt, $function, $receiver)
    );
    ($sel:expr, $rt:expr, $function:ident, $receiver:ident, $rhs:ident) => (
        binary_op_foreach!($sel, $rt, $function, $receiver, $rhs)
    );
    ($sel:expr, $rt:expr, $function:ident, $receiver:ident, $arg0:ident, $arg1:ident) => (
        ternary_op_foreach!($sel, $rt, $function, $receiver, $arg0, $arg1)
    );
    ($sel:expr, $rt:expr, $function:ident, $receiver:ident, $arg0:ident, $arg1:ident, $arg2:ident) => (
        _4ary_op_foreach!($sel, $rt, $function, $receiver, $arg0, $arg1, $arg2)
    );
}


macro_rules! expr_foreach_builtin {
    ($obj:expr, $inner:ident, $e:block) => (
       match $obj {
            &Builtin::Bool(ref $inner) => $e,
            &Builtin::None(ref $inner) => $e,
            &Builtin::Int(ref $inner) => $e,
            &Builtin::Float(ref $inner) => $e,
            &Builtin::Iter(ref $inner) => $e,
            &Builtin::Dict(ref $inner) => $e,
            &Builtin::Str(ref $inner) => $e,
            &Builtin::Bytes(ref $inner) => $e,
            &Builtin::Tuple(ref $inner) =>$e,
            &Builtin::List(ref $inner) =>$e,
            &Builtin::Function(ref $inner) => $e,
            &Builtin::Object(ref $inner) => $e,
            &Builtin::Type(ref $inner) => $e,
            &Builtin::Module(ref $inner) => $e,
            &Builtin::Code(ref $inner) => $e,
            &Builtin::Frame(ref $inner) => $e,
            _ => unreachable!()
        }
    );
}


macro_rules! unary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($rt),
            &Builtin::None(ref $lhs) => $lhs.$op($rt),
            &Builtin::Int(ref $lhs) => $lhs.$op($rt),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt),
            &Builtin::Iter(ref $lhs) => $lhs.$op($rt),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($rt),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt),
            &Builtin::List(ref $lhs) => $lhs.$op($rt),
            &Builtin::Function(ref $lhs) => $lhs.$op($rt),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt),
            &Builtin::Type(ref $lhs) => $lhs.$op($rt),
            &Builtin::Module(ref $lhs) => $lhs.$op($rt),
            &Builtin::Code(ref $lhs) => $lhs.$op($rt),
            &Builtin::Frame(ref $lhs) => $lhs.$op($rt),
            _ => unreachable!()
        }
    };
}


macro_rules! binary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::None(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Int(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Iter(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Function(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Type(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Module(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Code(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Frame(ref $lhs) => $lhs.$op($rt, $rhs),
            _ => unreachable!()
        }
    };
}


macro_rules! ternary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::None(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Int(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Iter(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Function(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Type(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Module(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Code(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Frame(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            _ => unreachable!()
        }
    };
}


macro_rules! _4ary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $arg0:ident, $arg1:ident, $arg2:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::None(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Int(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Iter(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::List(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Function(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Type(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Module(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Code(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Builtin::Frame(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            _ => unreachable!()
        }
    };
}


macro_rules! native_foreach_builtin {
    ($sel:expr, $function:ident, $receiver:ident) => (
        native_unary_op_foreach!($sel, $function, $receiver)
    );
    ($sel:expr, $function:ident, $receiver:ident, $rhs:ident) => (
        native_binary_op_foreach!($sel, $function, $receiver, $rhs)
    );
    ($sel:expr, $function:ident, $receiver:ident, $arg0:ident, $arg1:ident) => (
        native_ternary_op_foreach!($sel, $function, $receiver, $arg0, $arg1)
    );
    ($sel:expr, $function:ident, $receiver:ident, $arg0:ident, $arg1:ident, $arg2:ident) => (
        native_4ary_op_foreach!($sel, $function, $receiver, $arg0, $arg1, $arg2)
    )
}


macro_rules! native_unary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op(),
            &Builtin::None(ref $lhs) => $lhs.$op(),
            &Builtin::Int(ref $lhs) => $lhs.$op(),
            &Builtin::Float(ref $lhs) => $lhs.$op(),
            &Builtin::Iter(ref $lhs) => $lhs.$op(),
            &Builtin::Dict(ref $lhs) => $lhs.$op(),
            &Builtin::Str(ref $lhs) => $lhs.$op(),
            &Builtin::Bytes(ref $lhs) => $lhs.$op(),
            &Builtin::Tuple(ref $lhs) => $lhs.$op(),
            &Builtin::List(ref $lhs) => $lhs.$op(),
            &Builtin::Function(ref $lhs) => $lhs.$op(),
            &Builtin::Object(ref $lhs) => $lhs.$op(),
            &Builtin::Type(ref $lhs) => $lhs.$op(),
            &Builtin::Module(ref $lhs) => $lhs.$op(),
            &Builtin::Code(ref $lhs) => $lhs.$op(),
            &Builtin::Frame(ref $lhs) => $lhs.$op(),
            _ => unreachable!()
        }
    };
}


macro_rules! native_binary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($rhs),
            &Builtin::None(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Int(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Iter(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Function(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Type(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Module(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Code(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Frame(ref $lhs) => $lhs.$op($rhs),
            _ => unreachable!()
        }
    };
}


macro_rules! native_ternary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::None(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Int(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Float(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Iter(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::List(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Function(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Type(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Module(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Code(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Frame(ref $lhs) => $lhs.$op($mid, $rhs),
            _ => unreachable!()
        }
    };
}


macro_rules! native_4ary_op_foreach {
    ($obj:expr, $op:ident, $lhs:ident, $arg0:ident, $arg1:ident, $arg2:ident) => {
        match $obj {
            &Builtin::Bool(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::None(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Int(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Float(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Iter(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Dict(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Str(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Bytes(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Tuple(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::List(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Function(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Object(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Type(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Module(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Code(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Builtin::Frame(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            _ => unreachable!()
        }
    };
}


/// Macro to create Object and native typed level hooks for
/// the rsnek runtime. Each Function is generated with a default implementation
/// that will return a NotImplemented error.
///
/// Note that for arity of Functions may appear deceiving since the receiver (self)
/// is always the first argument and is the first argument by convention.
macro_rules! api_trait {
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel) -> NativeResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel) -> NativeResult<Builtin> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<Builtin> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (4ary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Builtin, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
    (variadic, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &Vec<ObjectRef>) -> RuntimeResult {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }

            fn $nfname(&$sel, &Vec<Builtin>) -> NativeResult<Builtin> {
                Err(Error::system_not_implemented(stringify!($pyname)))
            }
        }
    };
}


macro_rules! api_test_stub {
    ($args:ident, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        //#[test]
        fn $pyname() {
            println!("[stub] {} {} {} {} {}", stringify!($args), stringify!($pyname), stringify!($tname), stringify!($fname), stringify!($nfname));
            unimplemented!()
        }
    };
    ($args:ident, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $($misc:ty),*) => {
        api_test_stub!($args, $sel, $pyname, $tname, $fname, $nfname);
    };
}


// Errors that should be in resource::strings but constant format strings are
// kind of an edge case I guess.
macro_rules! strings_error_bad_operand {
    ($op:expr, $lhs:expr, $rhs:expr) => {
        format!("unsupported operand type(s) for {}: '{}' and '{}'", $op, $lhs, $rhs);
    }
}

macro_rules! strings_error_no_attribute {
    ($obj:expr, $attr:expr) => {
        format!("'{}' has no attribute '{:?}'", $obj, $attr);
    }
}

macro_rules! string_error_bad_attr_type {
    ($expect:expr, $actual:expr) => {
        &format!("attribute type must be '{}' not '{}'", $expect, $actual)
    }
}

