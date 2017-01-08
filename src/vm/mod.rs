pub mod class;
pub mod constant_pool;
pub mod frame;
pub mod sig;
pub mod symref;

pub use self::constant_pool::ConstantPool;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(Rc<RefCell<class::Class>>),
}
