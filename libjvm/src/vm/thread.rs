use crate::vm::opcode::Op;
use crate::vm::stack::Stack;
use crate::vm::types::NativeValue::Integer;

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
    pub fn execute(&mut self) {
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
            Op::AConstNull => {}
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
            Op::CheckCast(_) => {}
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
            Op::Dup => {}
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
            Op::I2B => {}
            Op::I2C => {}
            Op::I2D => {}
            Op::I2F => {}
            Op::I2L => {}
            Op::I2S => {}
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

    fn iadd(&mut self) {
        let stack = &mut self.stack.current_frame_mut().operand_stack;
        let op2 = stack.pop_integer();
        let op1 = stack.pop_integer();
        stack.push(Integer(op1 + op2))
    }

    fn imul(&mut self) {
        let stack = &mut self.stack.current_frame_mut().operand_stack;
        let op2 = stack.pop_integer();
        let op1 = stack.pop_integer();
        stack.push(Integer(op1 * op2))
    }
}
