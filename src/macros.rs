use std;

use std::io::Write;


// TODO: Replace this with an actual logging mod
macro_rules! log {
   ($fmt:expr) => (print!(concat!("DEBUG: ", $fmt, "\n")));
   ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG: ", $fmt, "\n"), $($arg)*));
}

macro_rules! try_cast {
    ($out:ident, $objref:expr, $builtin:path) => (
        let boxed: &Box<Builtin> = $objref.0.borrow();
        $out = match *boxed {
            $builtin(ref obj) => &obj,
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
    )
}

macro_rules! unary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op($rt),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt),
//            &Builtin::None(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Integer(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Float(ref $lhs) => $lhs.$op($rt),
//            &Builtin::String(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt),
//            &Builtin::List(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Set(ref $lhs) => $lhs.$op($rt),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Complex(ref $lhs) => $lhs.$op($rt),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt),
            _ => unreachable!()
        }
    };
}

macro_rules! binary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt, $rhs),
//            
//            &Builtin::None(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Integer(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::String(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::List(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Set(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Complex(ref $lhs) => $lhs.$op($rt, $rhs),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt, $rhs),
            _ => unreachable!()
        }
    };
}

macro_rules! ternary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::None(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Integer(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Float(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::String(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::List(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Set(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Complex(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
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
    )

}

macro_rules! native_unary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op(),
            &Builtin::Dict(ref $lhs) => $lhs.$op(),
            &Builtin::Str(ref $lhs) => $lhs.$op(),
            &Builtin::Object(ref $lhs) => $lhs.$op(),
                
//            &Builtin::None(ref $lhs) => $lhs.$op(),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op(),
//            &Builtin::Integer(ref $lhs) => $lhs.$op(),
//            &Builtin::Float(ref $lhs) => $lhs.$op(),
//            &Builtin::String(ref $lhs) => $lhs.$op(),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op(),
//            &Builtin::List(ref $lhs) => $lhs.$op(),
//            &Builtin::Set(ref $lhs) => $lhs.$op(),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op(),
//            &Builtin::Complex(ref $lhs) => $lhs.$op(),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op(),
            _ => unreachable!()
        }
    };
}

macro_rules! native_binary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::None(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Integer(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Float(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::String(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::List(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Set(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Complex(ref $lhs) => $lhs.$op($rhs),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op($rhs),
            _ => unreachable!()
        }
    };
}

macro_rules! native_ternary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Builtin::Int(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Dict(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Str(ref $lhs) => $lhs.$op($mid, $rhs),
            &Builtin::Object(ref $lhs) => $lhs.$op($mid, $rhs),

//            &Builtin::None(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Boolean(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Integer(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Float(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::String(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Tuple(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::List(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Set(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::FrozenSet(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Complex(ref $lhs) => $lhs.$op($mid, $rhs),
//            &Builtin::Dictionary(ref $lhs) => $lhs.$op($mid, $rhs),
            _ => unreachable!()
        }
    };
}



/// Macro to create Object and native typed level hooks for
/// the rsnek runtime. Each method is generated with a default implementation
/// that will return a NotImplemented error.
///
/// Note that for arity of methods may appear deceiving since the receiver (self)
/// is always the first argument and is the first argument by convention.
macro_rules! api_method {
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }
    };
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }

    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
    (4ary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {

            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
    (variadic, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, &Runtime, &Vec<ObjectRef>) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Vec<Builtin>) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
    };
}



/// Macro to create Object and native typed level hooks for
/// the rsnek runtime. Each method is generated with a default implementation
/// that will return a NotImplemented error.
///
/// Note that for arity of methods may appear deceiving since the receiver (self)
/// is always the first argument and is the first argument by convention.
macro_rules! api_trait {
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }
        }
    };
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<$nativety> {
                Err(Error::not_implemented())
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
        }
    };
    (4ary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Builtin, &Builtin, &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
        }
    };
    (variadic, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            fn $fname(&$sel, &Runtime, &Vec<ObjectRef>) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, &Vec<Builtin>) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
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
