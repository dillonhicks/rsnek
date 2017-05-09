#![feature(test)]
extern crate test;
extern crate rsnek;
extern crate itertools;

use itertools::Itertools;
use test::Bencher;


use rsnek::compiler::{Compiler, CompilerResult, Instr};
use rsnek::runtime::{Runtime, Interpreter};


pub fn setup<'a>(code: &str) ->  Box<[Instr]> {
    match Compiler::new().compile_str(&code) {
        Ok(ins) => ins,
        Err(_) => {
            panic!("SyntaxError: Unable to compile input");
        }
    }
}

macro_rules! bench_snippet (
    ($name:ident, $code:expr) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let rt = Runtime::new();
            let mut interpreter = Interpreter::new(&rt);
            let instructions = setup($code);
            b.iter(|| interpreter.exec(&rt, &(*instructions)).unwrap());
        }
    );
);


#[cfg(test)]
mod expr_call {
    use super::*;

    bench_snippet!(print_const_0001, "print(1)");
    bench_snippet!(print_const_0005, "print(print(print(print(print(1)))))");
}

#[cfg(test)]
mod stmt_assign {
    use super::*;

    bench_snippet!(assign_x_to_list_0000, "x = []");
    bench_snippet!(assign_x_to_list_0001, "x = [1]");
    bench_snippet!(assign_x_to_list_0002, "x = [1, 2]");
    bench_snippet!(assign_x_to_list_0004, "x = [1, 2, 3, 4]");
}

#[cfg(test)]
mod expr_binop {
    use super::*;
    // Cases that COULD be optimized at compile time to a constant
    bench_snippet!(_0000_str_plus_str,          r#"'things' + 'otherthings'"#);
    bench_snippet!(_0001_static_int_plus_int,   r#"1 + 345"#);
    bench_snippet!(_0002_dynamic_int_plus_int,  r#"100000000 + 34500000000"#);
    bench_snippet!(_0003_float_plus_int,        r#"'things' + 'otherthings'"#);
    bench_snippet!(_0004_float_plus_float,      r#"345.23 + 2123444.33333"#);
    bench_snippet!(_0005_lshift_int_34,         r#"1 << 34"#);
    bench_snippet!(_0006_lshift_int_4096,       r#"1 << 4096"#);
    bench_snippet!(_0007_str_mul_00,            r#"'stringtomultiply' * 0"#);
    bench_snippet!(_0008_str_mul_01,            r#"'stringtomultiply' * 1"#);
    bench_snippet!(_0009_str_mul_02,            r#"'stringtomultiply' * 2"#);
    bench_snippet!(_0010_str_mul_04,            r#"'stringtomultiply' * 4"#);
    bench_snippet!(_0011_str_mul_08,            r#"'stringtomultiply' * 8"#);
    bench_snippet!(_0012_str_mul_16,            r#"'stringtomultiply' * 16"#);
    bench_snippet!(_0013_str_mul_64,            r#"'stringtomultiply' * 64"#);
    bench_snippet!(_0014_list_mul_00,           r#"[0,1,2,3,4,5,6,7,8,9] * 0"#);
    bench_snippet!(_0015_list_mul_01,           r#"[0,1,2,3,4,5,6,7,8,9] * 1"#);
    bench_snippet!(_0016_list_mul_02,           r#"[0,1,2,3,4,5,6,7,8,9] * 2"#);
    bench_snippet!(_0017_list_mul_04,           r#"[0,1,2,3,4,5,6,7,8,9] * 4"#);
    bench_snippet!(_0018_list_mul_08,           r#"[0,1,2,3,4,5,6,7,8,9] * 8"#);
    bench_snippet!(_0019_list_mul_16,           r#"[0,1,2,3,4,5,6,7,8,9] * 16"#);
    bench_snippet!(_0020_list_mul_64,           r#"[0,1,2,3,4,5,6,7,8,9] * 64"#);
}
