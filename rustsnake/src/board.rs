use crate::snake::Snake;

pub struct Board {
    board: Vec<Vec<char>>,
    pub food: (usize, usize),
}

pub const ROW: usize = 20;
pub const COLUMN: usize = 40;

impl Board {
    pub fn new() -> Self {
        Board {
            board: vec![vec![' '; COLUMN]; ROW],
            food: (8, 8),
        }
    }
    pub fn draw(&mut self, snake: &mut Snake) {
        if self.food == snake.head_to {
            snake.lengthen = true;
            self.make_food();
        }

        self.board
            .iter_mut()
            .enumerate()
            .for_each(|(row_pos, row)| {
                row.iter_mut().enumerate().for_each(|(col_pos, cell)| {
                    if row_pos == 0 && col_pos == 0 {
                        *cell = '0'
                    } else if row_pos == 0 || row_pos == ROW - 1 || col_pos == 0 || col_pos == COLUMN - 1 {
                        *cell = '+'
                    } else if snake.body.contains(&(row_pos, col_pos)) {
                        *cell = 'X'
                    } else if self.food == (row_pos, col_pos) {
                        *cell = '@'
                    } else {
                        *cell = ' '
                    }
                })
            });

        self.board.iter().for_each(|row| {
            row.iter().for_each(|cell| print!("{}", cell));
            print!("\n");
        });
    }

    fn make_food(&mut self) {
        let border = 5;
        self.food = (
            rand::random::<usize>() % (ROW - border * 2) + border,
            rand::random::<usize>() % (COLUMN - border * 2) + border,
        );
        println!("new food: ({}, {})", self.food.0, self.food.1);
    }
}
