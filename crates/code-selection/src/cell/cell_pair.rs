use crate::*;

pub struct CellPair<'a> {
    pub main: &'a mut CellState,
    pub neighbor: &'a mut CellState,
    pub cycles_to_run: usize,
}

impl<'a> CellPair<'a> {
    pub fn new(main: &'a mut CellState, neighbor: &'a mut CellState) -> Self {
        Self {
            main,
            neighbor,
            cycles_to_run: 37,
        }
    }

    /// Handle single instruction execution.
    #[inline(always)]
    pub fn tick(&mut self) {
        loop {
            let instruction = self.read_instruction();
            instruction.process(self);

            if self.cycles_to_run == 0 {
                break;
            }

            self.cycles_to_run -= 1;
        }
    }

    pub fn read_instruction(&mut self) -> Instruction {
        let opcode = self.advance_pc();

        #[allow(clippy::unusual_byte_groupings)]
        match opcode {
            0b0000_0000 => InstructionNop.into(),

            // skip acc register (0x00 is nop) because `ld a a` does not make sense
            0b00000_001..=0b00000_111 => InstructionLoad::a_reg(opcode.into()).into(),
            0b00001_000..=0b00001_111 => InstructionLoad::atA_reg(opcode.into()).into(),
            0b00010_000..=0b00010_111 => InstructionLoad::reg_atA(opcode.into()).into(),
            0b00011_000 => InstructionLoad::a_byte.into(),
            // same as for InstructionLoad::a_reg (0b00000_001..=0b00000_111)
            0b00011_001..=0b00011_111 => InstructionLoad::reg_a(opcode.into()).into(),

            0b00100_000..=0b00100_111 => InstructionAdd::a_reg(opcode.into()).into(),
            0b00101_000..=0b00101_111 => InstructionAdd::a_atReg(opcode.into()).into(),

            0b00110_000..=0b00110_111 => InstructionSub::a_reg(opcode.into()).into(),
            0b00111_000..=0b00111_111 => InstructionSub::a_atReg(opcode.into()).into(),

            0b01000_000..=0b01000_111 => InstructionAnd::a_reg(opcode.into()).into(),
            0b01001_000..=0b01001_111 => InstructionAnd::a_atReg(opcode.into()).into(),

            0b01010_000..=0b01010_111 => InstructionOr::a_reg(opcode.into()).into(),
            0b01011_000..=0b01011_111 => InstructionOr::a_atReg(opcode.into()).into(),

            0b01100_000..=0b01100_111 => InstructionXor::a_reg(opcode.into()).into(),
            0b01101_000..=0b01101_111 => InstructionXor::a_atReg(opcode.into()).into(),

            0b01110_000..=0b01110_111 => InstructionNot::reg(opcode.into()).into(),
            0b01111_000..=0b01111_111 => InstructionNot::atReg(opcode.into()).into(),

            0b10000_000..=0b10000_111 => InstructionJump::reg(opcode.into()).into(),
            0b10001_000..=0b10001_111 => InstructionJump::atReg(opcode.into()).into(),
            0b10010_000..=0b10010_111 => InstructionJump::ifZ_reg(opcode.into()).into(),
            0b10011_000..=0b10011_111 => InstructionJump::ifZ_atReg(opcode.into()).into(),

            0b10100_000..=0b10100_111 => InstructionPush::reg(opcode.into()).into(),
            0b10101_000..=0b10101_111 => InstructionPush::atReg(opcode.into()).into(),

            0b10110_000..=0b10110_111 => InstructionPop::reg(opcode.into()).into(),
            0b10111_000..=0b10111_111 => InstructionPop::atReg(opcode.into()).into(),

            0b11000_000..=0b11000_111 => InstructionCall::reg(opcode.into()).into(),
            0b11001_000..=0b11001_111 => InstructionCall::ifZ_reg(opcode.into()).into(),

            0b11010_000..=0b11010_111 => InstructionLeftShift::reg(opcode.into()).into(),
            0b11011_000..=0b11011_111 => InstructionLeftShift::atReg(opcode.into()).into(),

            0b11100_000..=0b11100_111 => InstructionRightShift::reg(opcode.into()).into(),
            0b11101_000..=0b11101_111 => InstructionRightShift::atReg(opcode.into()).into(),

            0b11110_000 => InstructionCompare::a_byte.into(),
            0b11110_001..=0b11110_111 => InstructionCompare::a_reg(opcode.into()).into(),
            0b11111_000 => InstructionCompare::atA_byte.into(),

            0b11111_001 => InstructionReplicate.into(),

            0b11111_010 => InstructionJump::byte { if_z: false }.into(),
            0b11111_011 => InstructionJump::byte { if_z: true }.into(),

            0b11111_100 => InstructionCall::byte { if_z: true }.into(),
            0b11111_101 => InstructionCall::byte { if_z: false }.into(),

            0b111_11_110 => InstructionRet { if_z: false }.into(),
            0b111_11_111 => InstructionRet { if_z: true }.into(),
        }
    }

    /// Get the value of the memory cell at the given address.
    #[inline(always)]
    pub fn get_memory(&self, address: u8) -> u8 {
        if address < CellState::MEMORY_SIZE as u8 {
            self.main.memory[address as usize]
        } else {
            self.neighbor.memory[address as usize - CellState::MEMORY_SIZE]
        }
    }

    #[inline(always)]
    pub fn get_memory_at_acc(&self) -> u8 {
        self.get_memory(self.get_reg_acc())
    }

    #[inline(always)]
    pub fn get_memory_at_reg(&self, register: Register) -> u8 {
        self.get_memory(self.get_reg(register))
    }

    /// Set the value of the memory cell at the given address.
    #[inline(always)]
    pub fn set_memory(&mut self, address: u8, value: u8) {
        if address < CellState::MEMORY_SIZE as u8 {
            self.main.memory[address as usize] = value;
        } else {
            self.neighbor.memory[address as usize - CellState::MEMORY_SIZE] = value;
        }
    }

    #[inline(always)]
    pub fn set_memory_at_acc(&mut self, value: u8) {
        self.set_memory(self.get_reg_acc(), value);
    }

    #[inline(always)]
    pub fn set_memory_at_reg(&mut self, register: Register, value: u8) {
        self.set_memory(self.get_reg(register), value);
    }

    /// Return the value at the current program counter and advance it.
    #[inline(always)]
    pub fn advance_pc(&mut self) -> u8 {
        let result = self.get_memory(self.main.registers[CellState::REGISTER_PROGRAM_COUNTER]);

        self.main.registers[CellState::REGISTER_PROGRAM_COUNTER] =
            self.main.registers[CellState::REGISTER_PROGRAM_COUNTER].wrapping_add(1);

        result
    }

    /// Decrease the stack pointer and write the value at the new address.
    #[inline(always)]
    pub fn push_to_stack(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_STACK_POINTER] =
            self.main.registers[CellState::REGISTER_STACK_POINTER].wrapping_sub(1);
        self.set_memory(
            self.main.registers[CellState::REGISTER_STACK_POINTER],
            value,
        );
    }

    /// Increase the stack pointer and return the value at the new address.
    #[inline(always)]
    pub fn pop_from_stack(&mut self) -> u8 {
        let result = self.get_memory(self.main.registers[CellState::REGISTER_STACK_POINTER]);

        self.main.registers[CellState::REGISTER_STACK_POINTER] =
            self.main.registers[CellState::REGISTER_STACK_POINTER].wrapping_add(1);

        result
    }

    #[inline(always)]
    pub fn get_reg_acc(&self) -> u8 {
        self.main.registers[CellState::REGISTER_ACCUMULATOR]
    }

    #[inline(always)]
    pub fn get_reg_flags(&self) -> u8 {
        self.main.registers[CellState::REGISTER_FLAGS]
    }

    #[inline(always)]
    pub fn get_reg_pc(&self) -> u8 {
        self.main.registers[CellState::REGISTER_PROGRAM_COUNTER]
    }

    #[inline(always)]
    pub fn get_reg_sp(&self) -> u8 {
        self.main.registers[CellState::REGISTER_STACK_POINTER]
    }

    #[inline(always)]
    pub fn get_reg_b(&self) -> u8 {
        self.main.registers[CellState::REGISTER_B]
    }

    #[inline(always)]
    pub fn get_reg_c(&self) -> u8 {
        self.main.registers[CellState::REGISTER_C]
    }

    #[inline(always)]
    pub fn get_reg_d(&self) -> u8 {
        self.main.registers[CellState::REGISTER_D]
    }

    #[inline(always)]
    pub fn get_reg_e(&self) -> u8 {
        self.main.registers[CellState::REGISTER_E]
    }

    #[inline(always)]
    pub fn get_reg(&self, register: Register) -> u8 {
        match register {
            Register::Accumulator => self.main.registers[CellState::REGISTER_ACCUMULATOR],
            Register::Flags => self.main.registers[CellState::REGISTER_FLAGS],
            Register::ProgramCounter => self.main.registers[CellState::REGISTER_PROGRAM_COUNTER],
            Register::StackPointer => self.main.registers[CellState::REGISTER_STACK_POINTER],
            Register::B => self.main.registers[CellState::REGISTER_B],
            Register::C => self.main.registers[CellState::REGISTER_C],
            Register::D => self.main.registers[CellState::REGISTER_D],
            Register::E => self.main.registers[CellState::REGISTER_E],
        }
    }

    #[inline(always)]
    pub fn get_reg_mut(&mut self, register: Register) -> &mut u8 {
        match register {
            Register::Accumulator => &mut self.main.registers[CellState::REGISTER_ACCUMULATOR],
            Register::Flags => &mut self.main.registers[CellState::REGISTER_FLAGS],
            Register::ProgramCounter => {
                &mut self.main.registers[CellState::REGISTER_PROGRAM_COUNTER]
            }
            Register::StackPointer => &mut self.main.registers[CellState::REGISTER_STACK_POINTER],
            Register::B => &mut self.main.registers[CellState::REGISTER_B],
            Register::C => &mut self.main.registers[CellState::REGISTER_C],
            Register::D => &mut self.main.registers[CellState::REGISTER_D],
            Register::E => &mut self.main.registers[CellState::REGISTER_E],
        }
    }

    #[inline(always)]
    pub fn set_reg_acc(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_ACCUMULATOR] = value;
    }

    #[inline(always)]
    pub fn set_reg_flags(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_FLAGS] = value;
    }

    #[inline(always)]
    pub fn set_reg_pc(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_PROGRAM_COUNTER] = value;
    }

    #[inline(always)]
    pub fn set_reg_sp(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_STACK_POINTER] = value;
    }

    #[inline(always)]
    pub fn set_reg_b(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_B] = value;
    }

    #[inline(always)]
    pub fn set_reg_c(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_C] = value;
    }

    #[inline(always)]
    pub fn set_reg_d(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_D] = value;
    }

    #[inline(always)]
    pub fn set_reg_e(&mut self, value: u8) {
        self.main.registers[CellState::REGISTER_E] = value;
    }

    #[inline(always)]
    pub fn set_reg(&mut self, register: Register, value: u8) {
        match register {
            Register::Accumulator => self.set_reg_acc(value),
            Register::Flags => self.set_reg_flags(value),
            Register::ProgramCounter => self.set_reg_pc(value),
            Register::StackPointer => self.set_reg_sp(value),
            Register::B => self.set_reg_b(value),
            Register::C => self.set_reg_c(value),
            Register::D => self.set_reg_d(value),
            Register::E => self.set_reg_e(value),
        }
    }

    #[inline(always)]
    pub fn get_flag(&self, mask: u8) -> bool {
        self.get_reg_flags() & mask != 0
    }

    #[inline(always)]
    pub fn get_flag_z(&self) -> bool {
        self.get_flag(CellState::FLAG_Z_MASK)
    }

    #[inline(always)]
    pub fn get_flag_n(&self) -> bool {
        self.get_flag(CellState::FLAG_N_MASK)
    }

    #[inline(always)]
    pub fn get_flag_c(&self) -> bool {
        self.get_flag(CellState::FLAG_C_MASK)
    }

    #[inline(always)]
    pub fn set_flag(&mut self, mask: u8, value: bool) {
        if value {
            self.set_reg_flags(self.get_reg_flags() | mask);
        } else {
            self.set_reg_flags(self.get_reg_flags() & !mask);
        }
    }

    #[inline(always)]
    pub fn set_flag_z(&mut self, value: bool) {
        self.set_flag(CellState::FLAG_Z_MASK, value);
    }

    #[inline(always)]
    pub fn set_flag_n(&mut self, value: bool) {
        self.set_flag(CellState::FLAG_N_MASK, value);
    }

    #[inline(always)]
    pub fn set_flag_c(&mut self, value: bool) {
        self.set_flag(CellState::FLAG_C_MASK, value);
    }
}
