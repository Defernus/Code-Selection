use crate::*;
use macroquad::prelude::*;
use rayon::prelude::*;

pub struct World {
    pub size: AreaSize,
    pub cells: Vec<CellState>,
    pub update_stage: UpdateState,
    pub cell_cycles_per_tick: usize,
}

impl World {
    pub fn new(size: AreaSize) -> Self {
        assert!(size.width % 2 == 0, "World width must be even");
        assert!(size.height % 2 == 0, "World height must be even");

        Self {
            size,
            cells: (0..size.area()).map(|_| CellState::random()).collect(),
            update_stage: UpdateState::Vertical { reversed: false },
            cell_cycles_per_tick: 256,
        }
    }

    // TODO multithread this
    pub fn tick(&mut self) {
        let update_stage = self.update_stage;
        self.update_stage = update_stage.next();

        let mut pairs = Vec::<CellPair<'static>>::with_capacity(self.size.area() / 2);

        // update stage 0
        for i in update_stage.get_i_range(self.size) {
            for j in update_stage.get_j_range(self.size) {
                let (main_index, neighbor_index) = update_stage.get_indices(self.size, i, j);

                let (main_cell, neighbor_cell) =
                    get_pair_mut(&mut self.cells, main_index, neighbor_index);

                let pair = CellPair::new(main_cell, neighbor_cell);

                // Safety: we will drop the references before this function returns
                pairs.push(unsafe { std::mem::transmute::<CellPair<'_>, CellPair<'static>>(pair) });
            }
        }

        pairs.par_iter_mut().for_each(CellPair::tick);
    }

    /// Returns the size of the render area.
    pub fn get_image_size(&self) -> AreaSize {
        self.size * CellState::CANVAS_SIZE
    }

    pub fn draw_to_image(&self, image: &mut Image) {
        for (index, cell) in self.cells.iter().enumerate() {
            let pos = self.size.index_to_coords(index);

            let cell_pos = pos * CellState::CANVAS_SIZE;
            cell.draw_to_image(image, cell_pos);
        }

        image.set_pixel(0, 0, RED);
        image.set_pixel(
            (self.size.width * CellState::CANVAS_SIZE.width) as u32 - 1,
            (self.size.height * CellState::CANVAS_SIZE.height) as u32 - 1,
            GREEN,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    Vertical { reversed: bool },
    Horizontal { reversed: bool },
    VerticalOffset { reversed: bool },
    HorizontalOffset { reversed: bool },
}

impl UpdateState {
    pub fn next(self) -> Self {
        match self {
            Self::Vertical { reversed } => Self::Horizontal { reversed },
            Self::Horizontal { reversed } => Self::VerticalOffset { reversed },
            Self::VerticalOffset { reversed } => Self::HorizontalOffset { reversed },
            Self::HorizontalOffset { reversed } => Self::Vertical {
                reversed: !reversed,
            },
        }
    }

    pub fn get_i_range(self, world_size: AreaSize) -> std::ops::Range<u32> {
        match self {
            Self::Vertical { .. } | Self::VerticalOffset { .. } => 0..world_size.width as u32,
            Self::Horizontal { .. } | Self::HorizontalOffset { .. } => 0..world_size.height as u32,
        }
    }

    pub fn get_j_range(self, world_size: AreaSize) -> std::ops::Range<u32> {
        match self {
            Self::Vertical { .. } | Self::VerticalOffset { .. } => 0..world_size.height as u32 / 2,
            Self::Horizontal { .. } | Self::HorizontalOffset { .. } => {
                0..world_size.width as u32 / 2
            }
        }
    }

    pub fn get_indices(self, world_size: AreaSize, i: u32, j: u32) -> (usize, usize) {
        match self {
            UpdateState::Vertical { reversed } => {
                let x = i;

                let y0 = j * 2;
                let y1 = y0 + 1;

                let index0 = world_size.coords_to_index(RelativePosition::new(x, y0));
                let index1 = world_size.coords_to_index(RelativePosition::new(x, y1));

                if reversed {
                    (index1, index0)
                } else {
                    (index0, index1)
                }
            }
            UpdateState::VerticalOffset { reversed } => {
                let x = i;

                let y0 = j * 2 + 1;
                let y1 = (y0 + 1) % world_size.height as u32;

                let index0 = world_size.coords_to_index(RelativePosition::new(x, y0));
                let index1 = world_size.coords_to_index(RelativePosition::new(x, y1));

                if reversed {
                    (index1, index0)
                } else {
                    (index0, index1)
                }
            }
            UpdateState::Horizontal { reversed } => {
                let y = i;

                let x0 = j * 2;
                let x1 = x0 + 1;

                let index0 = world_size.coords_to_index(RelativePosition::new(x0, y));
                let index1 = world_size.coords_to_index(RelativePosition::new(x1, y));

                if reversed {
                    (index1, index0)
                } else {
                    (index0, index1)
                }
            }
            UpdateState::HorizontalOffset { reversed } => {
                let y = i;

                let x0 = j * 2 + 1;
                let x1 = (x0 + 1) % world_size.width as u32;

                let index0 = world_size.coords_to_index(RelativePosition::new(x0, y));
                let index1 = world_size.coords_to_index(RelativePosition::new(x1, y));

                if reversed {
                    (index1, index0)
                } else {
                    (index0, index1)
                }
            }
        }
    }
}
