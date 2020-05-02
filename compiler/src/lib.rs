macro_rules! w {
    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt(format_args!($($arg)*)).unwrap())
}

macro_rules! w_scope {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(format_args!($($arg)*)).unwrap();
        $dst.indent();
    }
}

macro_rules! w_end_scope {
    ($dst:expr, $($arg:tt)*) => {
        $dst.outdent();
        $dst.write_fmt(format_args!($($arg)*)).unwrap();
    };
}

pub use pecan_descriptor;

mod context;
mod descriptor;
mod generator;

pub use context::{Context, Output};
pub use generator::Generator;
