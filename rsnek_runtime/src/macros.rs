///! macros to make working with cases where there is generic code but not generic types
///! such as dispatching an API method on an `RtObject` or `Type`, creating method wrappers
///! (e.g. `x = 1; func = x.__hash__` since `__hash__` should be an object representing
///! `PyInteger::op_hash` for the instance ), shorthand for default implementations, etc.



/// Expands the `builtins::types::Type` into its variant to dispatch the given method
/// on that type.
macro_rules! foreach_type {
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

/// A more flexible sibling of the `foreach_type` and `native_foreach_type` macros
/// which will allow execution an arbitrary block of code on
/// the inner value of any variant of `Type`. The `$inner:ident` is
/// identifier used to reference the match expanded value within the given code block.
///
///```ignore
///  let object: RtObject = /// something that produces an RtObject;
///
///  expr_foreach_type!(object.as_ref(), value, {
///     write!(f, "{:?}", value)
/// })
/// ```
macro_rules! expr_foreach_type {
    ($obj:expr, $inner:ident, $e:block) => (
       match $obj {
            &Type::Bool(ref $inner) => $e,
            &Type::None(ref $inner) => $e,
            &Type::Int(ref $inner) => $e,
            &Type::Float(ref $inner) => $e,
            &Type::Iter(ref $inner) => $e,
            &Type::Dict(ref $inner) => $e,
            &Type::Str(ref $inner) => $e,
            &Type::Bytes(ref $inner) => $e,
            &Type::Tuple(ref $inner) =>$e,
            &Type::List(ref $inner) =>$e,
            &Type::Function(ref $inner) => $e,
            &Type::Object(ref $inner) => $e,
            &Type::Type(ref $inner) => $e,
            &Type::Module(ref $inner) => $e,
            &Type::Code(ref $inner) => $e,
            &Type::Frame(ref $inner) => $e,
            &Type::Set(ref $inner) => $e,
            &Type::FrozenSet(ref $inner) => $e,

            _ => unreachable!()
        }
    );
}


macro_rules! unary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($rt),
            &Type::None(ref $lhs) => $lhs.$op($rt),
            &Type::Int(ref $lhs) => $lhs.$op($rt),
            &Type::Float(ref $lhs) => $lhs.$op($rt),
            &Type::Iter(ref $lhs) => $lhs.$op($rt),
            &Type::Dict(ref $lhs) => $lhs.$op($rt),
            &Type::Str(ref $lhs) => $lhs.$op($rt),
            &Type::Bytes(ref $lhs) => $lhs.$op($rt),
            &Type::Tuple(ref $lhs) => $lhs.$op($rt),
            &Type::List(ref $lhs) => $lhs.$op($rt),
            &Type::Function(ref $lhs) => $lhs.$op($rt),
            &Type::Object(ref $lhs) => $lhs.$op($rt),
            &Type::Type(ref $lhs) => $lhs.$op($rt),
            &Type::Module(ref $lhs) => $lhs.$op($rt),
            &Type::Code(ref $lhs) => $lhs.$op($rt),
            &Type::Frame(ref $lhs) => $lhs.$op($rt),
            &Type::Set(ref $lhs) => $lhs.$op($rt),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($rt),

            _ => unreachable!()
        }
    };
}


macro_rules! binary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::None(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Int(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Float(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Iter(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Dict(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Str(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Bytes(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Tuple(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::List(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Function(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Object(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Type(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Module(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Code(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Frame(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::Set(ref $lhs) => $lhs.$op($rt, $rhs),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($rt, $rhs),

            _ => unreachable!()
        }
    };
}


macro_rules! ternary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::None(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Int(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Float(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Iter(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Dict(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Str(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Bytes(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Tuple(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::List(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Function(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Object(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Type(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Module(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Code(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Frame(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::Set(ref $lhs) => $lhs.$op($rt, $mid, $rhs),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($rt, $mid, $rhs),

            _ => unreachable!()
        }
    };
}


macro_rules! _4ary_op_foreach{
    ($obj:expr, $rt:expr, $op:ident, $lhs:ident, $arg0:ident, $arg1:ident, $arg2:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::None(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Int(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Float(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Iter(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Dict(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Str(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Bytes(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Tuple(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::List(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Function(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Object(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Type(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Module(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Code(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Frame(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::Set(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($rt, $arg0, $arg1, $arg2),

            _ => unreachable!()
        }
    };
}


macro_rules! native_foreach_type {
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
            &Type::Bool(ref $lhs) => $lhs.$op(),
            &Type::None(ref $lhs) => $lhs.$op(),
            &Type::Int(ref $lhs) => $lhs.$op(),
            &Type::Float(ref $lhs) => $lhs.$op(),
            &Type::Iter(ref $lhs) => $lhs.$op(),
            &Type::Dict(ref $lhs) => $lhs.$op(),
            &Type::Str(ref $lhs) => $lhs.$op(),
            &Type::Bytes(ref $lhs) => $lhs.$op(),
            &Type::Tuple(ref $lhs) => $lhs.$op(),
            &Type::List(ref $lhs) => $lhs.$op(),
            &Type::Function(ref $lhs) => $lhs.$op(),
            &Type::Object(ref $lhs) => $lhs.$op(),
            &Type::Type(ref $lhs) => $lhs.$op(),
            &Type::Module(ref $lhs) => $lhs.$op(),
            &Type::Code(ref $lhs) => $lhs.$op(),
            &Type::Frame(ref $lhs) => $lhs.$op(),
            &Type::Set(ref $lhs) => $lhs.$op(),
            &Type::FrozenSet(ref $lhs) => $lhs.$op(),

            _ => unreachable!()
        }
    };
}


macro_rules! native_binary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $rhs:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($rhs),
            &Type::None(ref $lhs) => $lhs.$op($rhs),
            &Type::Int(ref $lhs) => $lhs.$op($rhs),
            &Type::Float(ref $lhs) => $lhs.$op($rhs),
            &Type::Iter(ref $lhs) => $lhs.$op($rhs),
            &Type::Dict(ref $lhs) => $lhs.$op($rhs),
            &Type::Str(ref $lhs) => $lhs.$op($rhs),
            &Type::Bytes(ref $lhs) => $lhs.$op($rhs),
            &Type::Tuple(ref $lhs) => $lhs.$op($rhs),
            &Type::List(ref $lhs) => $lhs.$op($rhs),
            &Type::Function(ref $lhs) => $lhs.$op($rhs),
            &Type::Object(ref $lhs) => $lhs.$op($rhs),
            &Type::Type(ref $lhs) => $lhs.$op($rhs),
            &Type::Module(ref $lhs) => $lhs.$op($rhs),
            &Type::Code(ref $lhs) => $lhs.$op($rhs),
            &Type::Frame(ref $lhs) => $lhs.$op($rhs),
            &Type::Set(ref $lhs) => $lhs.$op($rhs),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($rhs),

            _ => unreachable!()
        }
    };
}


macro_rules! native_ternary_op_foreach{
    ($obj:expr, $op:ident, $lhs:ident, $mid:ident, $rhs:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::None(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Int(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Float(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Iter(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Dict(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Str(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Bytes(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Tuple(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::List(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Function(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Object(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Type(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Module(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Code(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Frame(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::Set(ref $lhs) => $lhs.$op($mid, $rhs),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($mid, $rhs),

            _ => unreachable!()
        }
    };
}


macro_rules! native_4ary_op_foreach {
    ($obj:expr, $op:ident, $lhs:ident, $arg0:ident, $arg1:ident, $arg2:ident) => {
        match $obj {
            &Type::Bool(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::None(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Int(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Float(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Iter(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Dict(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Str(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Bytes(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Tuple(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::List(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Function(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Object(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Type(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Module(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Code(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Frame(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::Set(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),
            &Type::FrozenSet(ref $lhs) => $lhs.$op($arg0, $arg1, $arg2),

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
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel) -> RtResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel) -> RtResult<Type> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &RtObject) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Type) -> RtResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &RtObject) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Type) -> RtResult<Type> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident, $nativety:ty) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &RtObject, &RtObject) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Type, &Type) -> RtResult<$nativety> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &RtObject, &RtObject) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Type, &Type) -> RtResult<Type> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (4ary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &RtObject, &RtObject, &RtObject) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Type, &Type, &Type) -> RtResult<Type> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
    (variadic, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        pub trait $tname {
            /// Runtime API Method $pyname
            fn $fname(&$sel, &Runtime, &Vec<RtObject>) -> ObjectResult {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }

            /// Native API Method $pyname
            fn $nfname(&$sel, &Vec<Type>) -> RtResult<Type> {
                Err(Error::system_not_implemented(stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) ))
            }
        }
    };
}


macro_rules! api_test_stub {
    ($args:ident, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
        //#[test]
        fn $pyname() {
            trace!("[stub] {} {} {} {} {}", stringify!($args), stringify!($pyname), &format!("file: {}, line: {}", file!(), line!()) , stringify!($tname), stringify!($fname), stringify!($nfname));
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

macro_rules! rsnek_exception_index {
    ($typ:expr) => {
        Error::index(&format!("{} {}", $typ, strings::ERROR_INDEX_OUT_OF_RANGE))
    }
}


macro_rules! unary_method_wrapper (
    ($sel:ident, $tname:expr, $fname:ident, $rt:ident, $builtin:path, $func:ident) => ({
        let selfref = $sel.rc.upgrade()?;
        let callable: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| {
            let object = selfref.clone();
            check_args(0, &pos_args)?;
            check_args(0, &starargs)?;
            check_kwargs(0, &kwargs)?;

            match object.as_ref() {
                &$builtin(ref value) => {
                $func(value, rt)
                }
                _ => unreachable!()
            }
        });

        Ok($rt.function(native::Func {
            name: format!("'{}' of {} object", $fname, $tname),
            signature: [].as_args(),
            module: strings::BUILTINS_MODULE.to_string(),
            callable: native::FuncType::MethodWrapper($sel.rc.upgrade()?, callable)
        }))

    });
);


macro_rules! binary_method_wrapper (
    ($sel:ident, $tname:expr, $fname:ident, $rt:ident, $builtin:path, $func:ident) => ({
        let selfref = $sel.rc.upgrade()?;
        let callable: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| {
            let object = selfref.clone();
            check_args(1, &pos_args)?;
            check_args(0, &starargs)?;
            check_kwargs(0, &kwargs)?;

            let arg = pos_args.op_getitem(&rt, &rt.int(0))?;

            match object.as_ref() {
                &$builtin(ref value) => {
                $func(value, rt, &arg)
                }
                _ => unreachable!()
            }
        });

        Ok($rt.function(native::Func {
            name: format!("'{}' of {} object", $fname, $tname),
            signature: ["arg1"].as_args(),
            module: strings::BUILTINS_MODULE.to_string(),
            callable: native::FuncType::MethodWrapper($sel.rc.upgrade()?, callable)
        }))

    });
);


macro_rules! ternary_method_wrapper (
    ($sel:ident, $tname:expr, $fname:ident, $rt:ident, $builtin:path, $func:ident) => ({
        let selfref = $sel.rc.upgrade()?;
        let callable: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| {
            let object = selfref.clone();
            check_args(2, &pos_args)?;
            check_args(0, &starargs)?;
            check_kwargs(0, &kwargs)?;

            let arg1 = pos_args.op_getitem(&rt, &rt.int(0))?;
            let arg2 = pos_args.op_getitem(&rt, &rt.int(1))?;
            match object.as_ref() {
                &$builtin(ref value) => {
                $func(value, rt, &arg1, &arg2)
                }
                _ => unreachable!()
            }
        });

        Ok($rt.function(native::Func {
            name: format!("'{}' of {} object", $fname, $tname),
            signature: ["arg1", "arg2"].as_args(),
            module: strings::BUILTINS_MODULE.to_string(),
            callable: native::FuncType::MethodWrapper($sel.rc.upgrade()?, callable)
        }))

    });
);

/// Used to create default "not implemented" impls for the Objects.
/// As an example suppose there is a new type `PyDatabaseConnector` that
/// should not implement the context manager traits `::api::method::Enter`
/// and `::api::method::Exit`. Since `PyDatabaseConnector` must implement all traits
/// of `::api::PyAPI` but the default implementations already return a `Result::Err`
/// (specifically, `Err(Error::system_not_implemented(...)).` There are many
/// impl blocks that are empty.
///
///
/// This macro allows for these cases to be short-hand with the following:
///
/// ```ignore
/// use ::api::method;
///
/// method_not_implemented!(PyDatabaseConnector, Enter Exit);
/// ```
macro_rules! method_not_implemented {
  ($Type:ty, $($ApiTrait:ident)+) => {
    $(
        impl method::$ApiTrait for $Type {}
    )+
  };
}