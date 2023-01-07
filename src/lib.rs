mod shrub;
use const_default::ConstDefault;
pub use shrub::*;

mod blossom;
pub use blossom::*;

mod style_data;

pub mod leaves;

// reexport taffy
pub use taffy;

pub const fn default<T: ConstDefault>() -> T {
    T::DEFAULT
}
