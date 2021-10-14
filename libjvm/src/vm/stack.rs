use crate::classfile::ConstantPool;
use crate::vm::types::NativeValue;
use std::sync::Arc;

pub struct Stack {
    frames: Vec<Frame>,
}

impl Stack {
    pub fn allocate(stack_capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(stack_capacity),
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }
}

pub struct Frame {
    pub locals: Vec<NativeValue>,
    pub operand_stack: OperandStack,
    pub constant_pool: Arc<ConstantPool>,
}

impl Frame {
    pub fn allocate(
        num_locals: usize,
        operand_stack_size: usize,
        constant_pool: Arc<ConstantPool>,
    ) -> Self {
        Self {
            locals: Vec::with_capacity(num_locals),
            operand_stack: OperandStack::new(operand_stack_size),
            constant_pool,
        }
    }
}

pub struct OperandStack {
    inner: Vec<NativeValue>,
}

impl OperandStack {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: NativeValue) {
        self.inner.push(value)
    }

    pub fn last(&mut self) -> &NativeValue {
        self.inner.last().unwrap()
    }

    pub fn pop(&mut self) -> NativeValue {
        self.inner.pop().unwrap()
    }

    pub fn pop_boolean(&mut self) -> bool {
        match self.pop() {
            NativeValue::Boolean(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_byte(&mut self) -> i8 {
        match self.pop() {
            NativeValue::Byte(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_char(&mut self) -> u16 {
        match self.pop() {
            NativeValue::Char(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_short(&mut self) -> i16 {
        match self.pop() {
            NativeValue::Short(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_integer(&mut self) -> i32 {
        match self.pop() {
            NativeValue::Integer(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_long(&mut self) -> i64 {
        match self.pop() {
            NativeValue::Long(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_reference(&mut self) -> usize {
        match self.pop() {
            NativeValue::Reference(v) => v,
            _ => panic!("invalid type"),
        }
    }

    pub fn pop_return_address(&mut self) -> usize {
        match self.pop() {
            NativeValue::ReturnAddress(v) => v,
            _ => panic!("invalid type"),
        }
    }
}
