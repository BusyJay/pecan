macro_rules! w {
    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt(format_args!($($arg)*)).unwrap())
}

pub use pecan_descriptor;

mod context;
mod descriptor;
mod generator;

pub use context::{Context, Output};
pub use generator::Generator;
