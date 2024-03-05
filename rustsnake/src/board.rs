use crate::snake::Snake;

pub struct Board {
    board: Vec<Vec<char>>,
    food: (usize, usize),
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
            .for_each(|(index_x, row)| {
                row.iter_mut().enumerate().for_each(|(index_y, cell)| {
                    if index_x == 0 || index_x == 19 || index_y == 0 || index_y == 39 {
                        *cell = '+'
                    } else if snake.body.contains(&(index_x, index_y)) {
                        *cell = 'X'
                    } else if self.food == (index_x, index_y) {
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
        self.food = (
            rand::random::<usize>() % COLUMN,
            rand::random::<usize>() % ROW,
        );
        println!("new food: ({}, {})", self.food.0, self.food.1);
    }
}
