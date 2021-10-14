use crate::vm::opcode::Op;
use crate::vm::stack::{OperandStack, Stack};
use crate::vm::types::NativeValue::*;

pub struct Thread {
    /// The pc register of this thread. As per [`$2.5.1`], this
    /// is wide enough to hold a native pointer on the platform.
    ///
    /// [`$2.5.1`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.5.1
    pc: usize,
    /// The private thread stack, as specified by [`$2.5.2`].
    ///
    /// [`$2.5.2`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.5.2
    stack: Stack,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Stack::allocate(10),
        }
    }

    pub fn run_method(&mut self, _class_name: &'static str, _method_name: &'static str) {}

    fn execute(&mut self) {
        loop {
            self.pc += 1;
        }
        /*
           do {
               atomically calculate pc and fetch opcode at pc;
               if (operands) fetch operands;
               execute the action for the opcode;
           } while (there is more to do);
        */
    }

    fn evaluate(&mut self, op: Op) {
        match op {
            Op::AALoad => {}
            Op::AAStore => {}
            Op::AConstNull => self.a_const_null(),
            Op::ALoad(_) => {}
            Op::ANewArray(_) => {}
            Op::AReturn => {}
            Op::ArrayLength => {}
            Op::AStore(_) => {}
            Op::AThrow => {}
            Op::BALoad => {}
            Op::BAStore => {}
            Op::BIPush(_) => {}
            Op::CALoad => {}
            Op::CAStore => {}
            Op::CheckCast(index) => self.check_cast(index),
            Op::D2F => {}
            Op::D2I => {}
            Op::D2L => {}
            Op::DAdd => {}
            Op::DALoad => {}
            Op::DAStore => {}
            Op::DCmpG => {}
            Op::DCmpL => {}
            Op::DConst0 => {}
            Op::DConst1 => {}
            Op::DDiv => {}
            Op::DLoad(_) => {}
            Op::DLoad0 => {}
            Op::DLoad1 => {}
            Op::DLoad2 => {}
            Op::DLoad3 => {}
            Op::DMul => {}
            Op::DNeg => {}
            Op::DRem => {}
            Op::DReturn => {}
            Op::DStore(_) => {}
            Op::DSub => {}
            Op::Dup => self.dup(),
            Op::DupX1 => {}
            Op::DupX2 => {}
            Op::Dup2 => {}
            Op::Dup2X1 => {}
            Op::Dup2X2 => {}
            Op::F2D => {}
            Op::F2I => {}
            Op::F2L => {}
            Op::FAdd => {}
            Op::FALoad => {}
            Op::FAStore => {}
            Op::FCmpG => {}
            Op::FCmpL => {}
            Op::FConst0 => {}
            Op::FConst1 => {}
            Op::FConst2 => {}
            Op::FDiv => {}
            Op::FLoad(_) => {}
            Op::FMul => {}
            Op::FNeg => {}
            Op::FRem => {}
            Op::FReturn => {}
            Op::FStore(_) => {}
            Op::FSub => {}
            Op::GetField(_) => {}
            Op::GetStatic(_) => {}
            Op::Goto(_) => {}
            Op::GotoW(_) => {}
            Op::I2B => self.i2b(),
            Op::I2C => self.i2c(),
            Op::I2D => self.i2d(),
            Op::I2F => self.i2f(),
            Op::I2L => self.i2l(),
            Op::I2S => self.i2s(),
            Op::IAdd => self.iadd(),
            Op::IALoad => {}
            Op::IAnd => {}
            Op::IAStore => {}
            Op::IConstM1 => {}
            Op::IConst0 => {}
            Op::IConst1 => {}
            Op::IConst2 => {}
            Op::IConst3 => {}
            Op::IConst4 => {}
            Op::IConst5 => {}
            Op::IDiv => {}
            Op::IfACmpEq(_) => {}
            Op::IfACmpNe(_) => {}
            Op::IfICmpEq(_) => {}
            Op::IfICmpNe(_) => {}
            Op::IfICmpLt(_) => {}
            Op::IfICmpGe(_) => {}
            Op::IfICmpGt(_) => {}
            Op::IfICmpLe(_) => {}
            Op::IfEq(_) => {}
            Op::IfNe(_) => {}
            Op::IfLt(_) => {}
            Op::IfGe(_) => {}
            Op::IfGt(_) => {}
            Op::IfLe(_) => {}
            Op::IfNonNull(_) => {}
            Op::IfNull(_) => {}
            Op::IInc(_, _) => {}
            Op::ILoad(_) => {}
            Op::IMul => self.imul(),
            Op::INeg => {}
            Op::InstanceOf(_) => {}
            Op::InvokeDynamic(_) => {}
            Op::InvokeInterface(_, _) => {}
            Op::InvokeSpecial(_) => {}
            Op::InvokeStatic(_) => {}
            Op::InvokeVirtual(_) => {}
            Op::IOr => {}
            Op::IRem => {}
            Op::IReturn => {}
            Op::IShl => {}
            Op::IShr => {}
            Op::IStore(_) => {}
            Op::ISub => {}
            Op::IUShr => {}
            Op::IXor => {}
            Op::Jsr(_) => {}
            Op::JsrW(_) => {}
            Op::L2D => {}
            Op::L2F => {}
            Op::L2I => {}
            Op::LAdd => {}
            Op::LALoad => {}
            Op::LAnd => {}
            Op::LAStore => {}
            Op::LCmp => {}
            Op::LConst0 => {}
            Op::LConst1 => {}
            Op::LDC(_) => {}
            Op::LDCW(_) => {}
            Op::LDC2W(_) => {}
            Op::LDiv => {}
            Op::LLoad(_) => {}
            Op::LMul => {}
            Op::LNeg => {}
            Op::LookupSwitch { .. } => {}
            Op::LOr => {}
            Op::LRem => {}
            Op::LReturn => {}
            Op::LShl => {}
            Op::LShr => {}
            Op::LStore(_) => {}
            Op::LSub => {}
            Op::LUShr => {}
            Op::LXor => {}
            Op::MonitorEnter => {}
            Op::MonitorExit => {}
            Op::MultiANewArray(_, _) => {}
            Op::New(_) => {}
            Op::NewArray(_) => {}
            Op::Nop => {}
            Op::Pop => {}
            Op::Pop2 => {}
            Op::PutField(_) => {}
            Op::PutStatic(_) => {}
            Op::Ret(_) => {}
            Op::Return => {}
            Op::SALoad => {}
            Op::SAStore => {}
            Op::SIPush(_) => {}
            Op::Swap => {}
            Op::TableSwitch { .. } => {}
            Op::Wide => {}
            Op::Breakpoint => {}
        }
    }

    fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.stack.current_frame_mut().operand_stack
    }

    fn a_const_null(&mut self) {
        let stack = self.operand_stack_mut();
        stack.push(Reference(0));
    }

    fn check_cast(&mut self, index: u16) {
        let stack = self.operand_stack_mut();
        if let Reference(r) = *stack.last() {
            if r == 0 {
                return;
            }
        }

        let reference = stack.pop_reference();
        todo!("cast")
    }

    fn dup(&mut self) {
        let stack = self.operand_stack_mut();
        let dup = stack.last().clone();
        stack.push(dup);
    }

    fn i2b(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Byte(v as i8));
    }

    fn i2c(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Char(v as u16));
    }

    fn i2d(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Double(v as f64));
    }

    fn i2f(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Float(v as f32));
    }

    fn i2l(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Long(v as i64));
    }

    fn i2s(&mut self) {
        let stack = self.operand_stack_mut();
        let v = stack.pop_integer();
        stack.push(Short(v as i16));
    }

    fn iadd(&mut self) {
        let stack = self.operand_stack_mut();
        let op2 = stack.pop_integer();
        let op1 = stack.pop_integer();
        stack.push(Integer(op1.wrapping_add(op2)));
    }

    fn imul(&mut self) {
        let stack = self.operand_stack_mut();
        let op2 = stack.pop_integer();
        let op1 = stack.pop_integer();
        stack.push(Integer(op1.wrapping_mul(op2)));
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::classfile::{ConstantPool, ConstantPoolInfo};
    use crate::vm::stack::Frame;

    use super::*;

    macro_rules! setup_thread {
        ($op_stack_size:expr) => {
            setup_thread!($op_stack_size, ConstantPool::from(vec![]))
        };

        ($op_stack_size:expr,$cp:expr) => {{
            let mut t = Thread::new();
            let frame = Frame::allocate(0, $op_stack_size, Arc::new($cp));
            t.stack.push_frame(frame);
            t
        }};
    }

    #[test]
    fn test_setup_thread() {
        let mut t = setup_thread!(0);
        assert_eq!(0, t.pc);
        assert_eq!(0, t.stack.current_frame_mut().locals.len());
        assert_eq!(0, t.stack.current_frame_mut().constant_pool.len());
        assert_eq!(0, t.operand_stack_mut().len());
    }

    #[test]
    fn test_setup_thread_cp() {
        let mut t = setup_thread!(
            0,
            ConstantPool::from(vec![ConstantPoolInfo::Utf8Info {
                length: 5,
                bytes: "hello".as_bytes().into()
            }])
        );
        assert_eq!(0, t.pc);
        assert_eq!(0, t.stack.current_frame_mut().locals.len());
        assert_eq!(1, t.stack.current_frame_mut().constant_pool.len());
        assert_eq!(
            ConstantPoolInfo::Utf8Info {
                length: 5,
                bytes: "hello".as_bytes().into()
            },
            t.stack.current_frame_mut().constant_pool[0]
        );
        assert_eq!(0, t.operand_stack_mut().len());
    }

    #[test]
    fn test_a_const_null() {
        let mut t = setup_thread!(1);

        t.evaluate(Op::AConstNull);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Reference(0), operand_stack.pop());
    }

    #[test]
    fn test_dup() {
        let mut t = setup_thread!(2);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::Dup);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Integer(17), operand_stack.pop());
        assert_eq!(Integer(17), operand_stack.pop());
    }

    #[test]
    fn test_i2b() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2B);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Byte(17), operand_stack.pop());
    }

    #[test]
    fn test_i2b_truncate() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(250));
        t.evaluate(Op::I2B);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Byte(-6), operand_stack.pop());
    }

    #[test]
    fn test_i2c() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2C);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Char(17), operand_stack.pop());
    }

    #[test]
    fn test_i2c_truncate() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(70000));
        t.evaluate(Op::I2C);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Char(4464), operand_stack.pop());
    }

    #[test]
    fn test_i2d() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2D);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Double(17.0), operand_stack.pop());
    }

    #[test]
    fn test_i2d_negative() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(-17));
        t.evaluate(Op::I2D);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Double(-17.0), operand_stack.pop());
    }

    #[test]
    fn test_i2f() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2F);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Float(17.0), operand_stack.pop());
    }

    #[test]
    fn test_i2f_negative() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(-17));
        t.evaluate(Op::I2F);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Float(-17.0), operand_stack.pop());
    }

    #[test]
    fn test_i2l() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2L);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Long(17), operand_stack.pop());
    }

    #[test]
    fn test_i2s() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(17));
        t.evaluate(Op::I2S);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Short(17), operand_stack.pop());
    }

    #[test]
    fn test_i2s_truncate() {
        let mut t = setup_thread!(1);

        t.operand_stack_mut().push(Integer(40000));
        t.evaluate(Op::I2S);

        let operand_stack = t.operand_stack_mut();
        assert_eq!(Short(-25536), operand_stack.pop());
    }

    #[test]
    fn test_iadd() {
        let mut t = setup_thread!(2);

        t.operand_stack_mut().push(Integer(9));
        t.operand_stack_mut().push(Integer(18));
        t.evaluate(Op::IAdd);
        let operand_stack = t.operand_stack_mut();
        assert_eq!(1, operand_stack.len());
        assert_eq!(Integer(27), operand_stack.pop());
    }

    #[test]
    fn test_iadd_overflow() {
        let mut t = setup_thread!(2);

        t.operand_stack_mut().push(Integer(2147483647));
        t.operand_stack_mut().push(Integer(1));
        t.evaluate(Op::IAdd);
        let operand_stack = t.operand_stack_mut();
        assert_eq!(1, operand_stack.len());
        assert_eq!(Integer(-2147483648), operand_stack.pop());
    }

    #[test]
    fn test_imul() {
        let mut t = setup_thread!(2);

        t.operand_stack_mut().push(Integer(9));
        t.operand_stack_mut().push(Integer(18));
        t.evaluate(Op::IMul);
        let operand_stack = t.operand_stack_mut();
        assert_eq!(1, operand_stack.len());
        assert_eq!(Integer(162), operand_stack.pop());
    }

    #[test]
    fn test_imul_overflow() {
        let mut t = setup_thread!(2);

        t.operand_stack_mut().push(Integer(1763));
        t.operand_stack_mut().push(Integer(2487369));
        t.evaluate(Op::IMul);
        let operand_stack = t.operand_stack_mut();
        assert_eq!(1, operand_stack.len());
        assert_eq!(Integer(90264251), operand_stack.pop());
    }
}
