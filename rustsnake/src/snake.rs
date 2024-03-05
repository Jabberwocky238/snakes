use rand::random;

pub struct Snake {
    /// head = body[0]
    pub body: Vec<(usize, usize)>,
    /// y, x
    pub direction: (i32, i32),
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![(10, 10), (9, 10), (8, 10)],
            direction: (0, 1),
        }
    }

    pub fn tick(&mut self){
        let head = self.body[0];
        let mut before = head.clone();
        let mut after = head.clone();
        self.body.iter_mut().for_each(|block|{
            after = block.clone();
            *block = before.clone();
            before = after;
        });
        match self.direction.0 {
            1 => self.body[0].0 += 1,
            -1 => self.body[0].0 -= 1,
            _ => {}
        }
        match self.direction.1 {
            1 => self.body[0].1 += 1,
            -1 => self.body[0].1 -= 1,
            _ => {}
        }
    }

    pub fn draw(&self, board: &mut Vec<Vec<char>>) {
        board.iter_mut().enumerate().for_each(|(index_x, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(index_y, cell)| {
                    if index_x == 0 || index_x == 19 || index_y == 0 || index_y == 39 {
                        *cell = '+'
                    }
                    else if self.body.contains(&(index_x, index_y)) {
                        *cell = 'X'
                    }
                    else {
                        *cell = ' '
                    }
                })
        });
    }

    pub fn up(&mut self) {
        self.direction.1 = 0;
        self.direction.0 = -1;
    }
    pub fn down(&mut self) {
        self.direction.1 = 0;
        self.direction.0 = 1;
    }
    pub fn left(&mut self) {
        self.direction.1 = -1;
        self.direction.0 = 0;
    }
    pub fn right(&mut self) {
        self.direction.1 = 1;
        self.direction.0 = 0;
    }
}
