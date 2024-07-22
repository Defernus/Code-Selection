use crate::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop(InstructionNop),
    Load(InstructionLoad),
    Add(InstructionAdd),
    Sub(InstructionSub),
    And(InstructionAnd),
    Or(InstructionOr),
    Xor(InstructionXor),
    Not(InstructionNot),
    Inc(InstructionInc),
    Dec(InstructionDec),
    Jmp(InstructionJump),
    Push(InstructionPush),
    Pop(InstructionPop),
    Call(InstructionCall),
    Ret(InstructionRet),
    LeftShift(InstructionLeftShift),
    RightShift(InstructionRightShift),
    Compare(InstructionCompare),
    Replicate(InstructionReplicate),
}

#[enum_dispatch(Instruction)]
pub trait ProcessInstruction {
    fn process(&self, _state: &mut CellPair) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Accumulator,
    Flags,
    ProgramCounter,
    StackPointer,
    B,
    C,
    D,
    E,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => Register::Accumulator,
            1 => Register::Flags,
            2 => Register::ProgramCounter,
            3 => Register::StackPointer,
            4 => Register::B,
            5 => Register::C,
            6 => Register::D,
            7 => Register::E,
            value => unreachable!("Invalid register value: {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionNop;

impl ProcessInstruction for InstructionNop {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionLoad {
    /// Load value from given register to accumulator.
    a_reg(Register),
    /// Load value from memory at address pointed by accumulator to given register.
    reg_atA(Register),
    /// Load value from accumulator to given register.
    reg_a(Register),
    /// Load value from register to memory at address pointed by accumulator.
    atA_reg(Register),
    /// read byte at pc, increment pc and load byte to accumulator.
    a_byte,
}

impl ProcessInstruction for InstructionLoad {
    fn process(&self, state: &mut CellPair) {
        match *self {
            Self::a_byte => {
                let value = state.advance_pc();
                state.set_reg_acc(value);
            }
            Self::a_reg(register) => state.set_reg_acc(state.get_reg(register)),
            Self::atA_reg(register) => state.set_memory_at_acc(state.get_reg(register)),
            Self::reg_a(register) => state.set_reg(register, state.get_reg_acc()),
            Self::reg_atA(register) => state.set_reg(register, state.get_memory_at_acc()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionAdd {
    /// Add value from given register to accumulator.
    a_reg(Register),
    /// Add value from memory at address pointed by given register to accumulator.
    a_atReg(Register),
}

impl ProcessInstruction for InstructionAdd {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::a_atReg(reg) => state.get_memory_at_reg(reg),
            Self::a_reg(reg) => state.get_reg(reg),
        };

        let acc = state.get_reg_acc();
        let result = acc.wrapping_add(value);

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(value as u16 + acc as u16 > 0xFF);

        state.set_reg_acc(result);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionSub {
    /// acc = acc - $reg
    a_reg(Register),
    /// acc = acc - [$reg]
    a_atReg(Register),
}

impl ProcessInstruction for InstructionSub {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::a_atReg(reg) => state.get_memory_at_reg(reg),
            Self::a_reg(reg) => state.get_reg(reg),
        };

        let acc = state.get_reg_acc();
        let result = acc.wrapping_sub(value);

        state.set_flag_z(result == 0);
        state.set_flag_n(true);
        state.set_flag_c(value > acc);

        state.set_reg_acc(result);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionAnd {
    /// acc = acc & $reg
    a_reg(Register),
    /// acc = acc & [$reg]
    a_atReg(Register),
}

impl ProcessInstruction for InstructionAnd {
    fn process(&self, state: &mut CellPair) {
        let v = match *self {
            Self::a_reg(reg) => state.get_reg(reg),
            Self::a_atReg(reg) => state.get_memory_at_reg(reg),
        };

        let acc = state.get_reg_acc();
        let result = acc & v;

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(false);

        state.set_reg_acc(result);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionOr {
    /// acc = acc | $reg
    a_reg(Register),
    /// acc = acc | [$reg]
    a_atReg(Register),
}

impl ProcessInstruction for InstructionOr {
    fn process(&self, state: &mut CellPair) {
        let v = match *self {
            Self::a_reg(reg) => state.get_reg(reg),
            Self::a_atReg(reg) => state.get_memory_at_reg(reg),
        };

        let acc = state.get_reg_acc();
        let result = acc | v;

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(false);

        state.set_reg_acc(result);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionXor {
    /// acc = acc ^ $reg
    a_reg(Register),
    /// acc = acc ^ [$reg]
    a_atReg(Register),
}

impl ProcessInstruction for InstructionXor {
    fn process(&self, state: &mut CellPair) {
        let v = match *self {
            Self::a_reg(reg) => state.get_reg(reg),
            Self::a_atReg(reg) => state.get_memory_at_reg(reg),
        };

        let acc = state.get_reg_acc();
        let result = acc ^ v;

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(false);

        state.set_reg_acc(result);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionNot {
    /// $reg = ~$reg
    reg(Register),
    /// [$reg] = ~[$reg]
    atReg(Register),
}

impl ProcessInstruction for InstructionNot {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::reg(reg) => state.get_reg(reg),
            Self::atReg(reg) => state.get_memory_at_reg(reg),
        };

        let result = !value;

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(false);

        match *self {
            Self::reg(reg) => state.set_reg(reg, result),
            Self::atReg(reg) => state.set_memory_at_reg(reg, result),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionInc {
    /// $reg += 1
    reg(Register),
    /// [$reg] +=  1
    atReg(Register),
}

impl ProcessInstruction for InstructionInc {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::reg(reg) => state.get_reg(reg),
            Self::atReg(reg) => state.get_memory_at_reg(reg),
        };

        let result = value.wrapping_add(1);

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(result < value);

        match *self {
            Self::reg(reg) => {
                state.set_reg(reg, result);
            }
            Self::atReg(reg) => {
                state.set_memory_at_reg(reg, result);
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionDec {
    /// $reg -= 1
    reg(Register),
    /// [$reg] -= 1
    atReg(Register),
}

impl ProcessInstruction for InstructionDec {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::reg(reg) => state.get_reg(reg),
            Self::atReg(reg) => state.get_memory_at_reg(reg),
        };

        let result = value.wrapping_sub(1);

        state.set_flag_z(result == 0);
        state.set_flag_n(true);
        state.set_flag_c(result > value);

        match *self {
            Self::reg(reg) => {
                state.set_reg(reg, result);
            }
            Self::atReg(reg) => {
                state.set_memory_at_reg(reg, result);
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionJump {
    /// pc = $reg
    reg(Register),
    /// pc = [$reg]
    atReg(Register),
    /// if z { pc = $reg }
    ifZ_reg(Register),
    /// if z { pc = [$reg] }
    ifZ_atReg(Register),

    byte {
        if_z: bool,
    },
}

impl ProcessInstruction for InstructionJump {
    fn process(&self, state: &mut CellPair) {
        match *self {
            Self::byte { if_z: true } | Self::ifZ_reg(_) | Self::ifZ_atReg(_)
                if !state.get_flag_z() =>
            {
                return;
            }
            _ => {}
        }

        match *self {
            Self::byte { .. } => {
                let address = state.advance_pc();
                state.push_to_stack(state.get_reg_pc());
                state.set_reg_pc(address);
            }
            Self::reg(reg) | Self::ifZ_reg(reg) => {
                let address = state.get_reg(reg);
                state.push_to_stack(state.get_reg_pc());
                state.set_reg_pc(address);
            }
            Self::atReg(reg) | Self::ifZ_atReg(reg) => {
                let address = state.get_memory_at_reg(reg);
                state.push_to_stack(state.get_reg_pc());
                state.set_reg_pc(address);
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPush {
    /// stack[sp] = $reg
    reg(Register),
    /// stack[sp] = [$reg]
    atReg(Register),
}

impl ProcessInstruction for InstructionPush {
    fn process(&self, state: &mut CellPair) {
        match *self {
            Self::reg(reg) => {
                let value = state.get_reg(reg);
                state.push_to_stack(value);
            }
            Self::atReg(reg) => {
                let value = state.get_memory_at_reg(reg);
                state.push_to_stack(value);
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPop {
    /// $reg = stack[sp]
    reg(Register),
    /// $reg = stack[sp]
    atReg(Register),
}

impl ProcessInstruction for InstructionPop {
    fn process(&self, state: &mut CellPair) {
        match *self {
            Self::reg(reg) => {
                let value = state.pop_from_stack();
                state.set_reg(reg, value);
            }
            Self::atReg(reg) => {
                let value = state.pop_from_stack();
                state.set_memory_at_reg(reg, value);
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCall {
    /// store pc in stack and jump to $reg
    reg(Register),
    ifZ_reg(Register),
    byte {
        if_z: bool,
    },
}

impl ProcessInstruction for InstructionCall {
    fn process(&self, state: &mut CellPair) {
        match *self {
            Self::byte { if_z: true } | Self::ifZ_reg(_) if !state.get_flag_z() => return,
            _ => {}
        }

        let address = match *self {
            Self::byte { .. } => state.advance_pc(),
            Self::reg(reg) | Self::ifZ_reg(reg) => state.get_reg(reg),
        };

        state.push_to_stack(state.get_reg_pc());
        state.set_reg_pc(address);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionRet {
    pub if_z: bool,
}

impl ProcessInstruction for InstructionRet {
    fn process(&self, state: &mut CellPair) {
        if self.if_z && !state.get_flag_z() {
            return;
        }

        let address = state.pop_from_stack();
        state.set_reg_pc(address);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionLeftShift {
    /// Shift value in register to the left.
    reg(Register),
    /// Shift value in memory at address pointed by register to the left.
    atReg(Register),
}

impl ProcessInstruction for InstructionLeftShift {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::reg(reg) => state.get_reg(reg),
            Self::atReg(reg) => state.get_memory_at_reg(reg),
        };

        let result = value.wrapping_shl(1);

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(value & 0b1000_0000 != 0);

        match *self {
            Self::reg(reg) => state.set_reg(reg, result),
            Self::atReg(reg) => state.set_memory_at_reg(reg, result),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionRightShift {
    /// Shift value in register to the right.
    reg(Register),
    /// Shift value in memory at address pointed by register to the right.
    atReg(Register),
}

impl ProcessInstruction for InstructionRightShift {
    fn process(&self, state: &mut CellPair) {
        let value = match *self {
            Self::reg(reg) => state.get_reg(reg),
            Self::atReg(reg) => state.get_memory_at_reg(reg),
        };

        let result = value.wrapping_shr(1);

        state.set_flag_z(result == 0);
        state.set_flag_n(false);
        state.set_flag_c(value & 0b0000_0001 != 0);

        match *self {
            Self::reg(reg) => state.set_reg(reg, result),
            Self::atReg(reg) => state.set_memory_at_reg(reg, result),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCompare {
    /// Compare value in register with accumulator.
    a_reg(Register),
    /// Compare value in accumulator with next byte.
    a_byte,
    atA_byte,
}

impl ProcessInstruction for InstructionCompare {
    fn process(&self, state: &mut CellPair) {
        let (a, b) = match *self {
            Self::a_reg(reg) => (state.get_reg_acc(), state.get_reg(reg)),
            Self::a_byte => (state.get_reg_acc(), state.advance_pc()),
            Self::atA_byte => (state.get_memory_at_acc(), state.advance_pc()),
        };

        state.set_flag_z(a == b);
        state.set_flag_n(true);
        state.set_flag_c(a < b);
    }
}

/// Copy all data from main cell to neighbor cell.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionReplicate;

impl ProcessInstruction for InstructionReplicate {
    fn process(&self, _state: &mut CellPair) {
        // state.cycles_to_run = 0;
        // std::mem::swap(state.main, state.neighbor);

        // for _ in 0..8 {
        //     let mutate_address = ::rand::random::<u8>() as usize % CellState::MEMORY_SIZE;
        //     state.main.memory[mutate_address] = ::rand::random();
        // }
    }
}
