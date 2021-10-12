use crate::vm::area::{Heap, MethodArea};
use std::sync::{Arc, RwLock};

pub mod area;
pub mod opcode;
pub mod stack;
pub mod thread;
pub mod types;

pub struct VM {
    /// Specified by [`$2.5.3`].
    ///
    /// [`$2.5.3`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.5.3
    heap: Arc<RwLock<Heap>>,
    method_area: Arc<RwLock<MethodArea>>,
}
