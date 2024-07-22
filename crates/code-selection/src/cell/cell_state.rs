use crate::{AreaSize, RelativePosition};
use macroquad::{color::Color, texture::Image};

#[derive(Debug, Clone, Copy)]
pub struct CellState {
    /// Memory of the cell. At each simulation tick a random pair of adjacent cells is selected.
    pub memory: [u8; Self::MEMORY_SIZE],
    /// Accumulator, flags, program counter, stack pointer and 4 general purpose registers.
    pub registers: [u8; 8],
}

impl CellState {
    pub const CANVAS_SIZE: AreaSize = AreaSize {
        width: 7,
        height: 7,
    };
    pub const MEMORY_SIZE: usize = (u8::MAX as usize + 1) / 2;
    pub const REGISTER_ACCUMULATOR: usize = 0;
    pub const REGISTER_FLAGS: usize = 1;
    pub const REGISTER_PROGRAM_COUNTER: usize = 2;
    pub const REGISTER_STACK_POINTER: usize = 3;
    pub const REGISTER_B: usize = 4;
    pub const REGISTER_C: usize = 5;
    pub const REGISTER_D: usize = 6;
    pub const REGISTER_E: usize = 7;

    /// Zero flag
    pub const FLAG_Z_MASK: u8 = 0b0000_0001;
    /// Negative flag
    pub const FLAG_N_MASK: u8 = 0b0000_0010;
    /// Carry flag
    pub const FLAG_C_MASK: u8 = 0b0000_0100;

    pub fn random() -> Self {
        let mut memory = [0; Self::MEMORY_SIZE];
        for cell in memory.iter_mut() {
            *cell = ::rand::random();
        }

        let registers = ::rand::random();

        Self { memory, registers }
    }

    pub fn draw_to_image(&self, image: &mut Image, offset: RelativePosition) {
        let mut pixel_index = 0usize;

        macro_rules! set_pixel {
            ($r:expr, $g:expr, $b:expr) => {
                let pos = Self::CANVAS_SIZE.index_to_coords(pixel_index) + offset;
                image.set_pixel(pos.x, pos.y, Color::from_rgba($r, $g, $b, 255));
                pixel_index += 1;
            };
        }

        set_pixel!(self.registers[0], self.registers[1], self.registers[2]);
        set_pixel!(self.registers[3], self.registers[4], self.registers[5]);
        set_pixel!(self.registers[6], self.registers[7], 0);

        for i in 0..(Self::MEMORY_SIZE / 3 + 1) {
            let memory_offset = i * 3;

            let r = self.memory.get(memory_offset).copied().unwrap_or_default();
            let g = self
                .memory
                .get(memory_offset + 1)
                .copied()
                .unwrap_or_default();
            let b = self
                .memory
                .get(memory_offset + 2)
                .copied()
                .unwrap_or_default();

            set_pixel!(r, g, b);
        }
    }
}
