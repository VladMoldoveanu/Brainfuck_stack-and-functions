//! Contains the Operations, Arrays and Functions
//!
//! Takes care of the low-level instances and ensures their functionality.

mod base_operation;
mod function;
mod array_handler;
mod stack_handler;

pub use self::base_operation::Operation;
pub use self::base_operation::Operation::*;
pub use self::array_handler::ArrayHandler;
pub use self::function::FunctionHolder;