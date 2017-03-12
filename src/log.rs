use std;

use std::io::Write;

#[macro_export]
macro_rules! debug {
   () => (print!("DEBUG: \n"));
   ($fmt:expr) => (print!(concat!("DEBUG: ", $fmt, "\n")));
   ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG: ", $fmt, "\n"), $($arg)*));
}
