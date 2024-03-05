
pub struct Snake {
    /// head = body[0]
    pub body: Vec<(usize, usize)>,
    /// y, x
    pub direction: (i32, i32),
    /// head towards
    pub head_to: (usize, usize),
    /// 吃了吗
    pub lengthen: bool,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![(10, 10), (9, 10), (8, 10)],
            direction: (0, 1),
            head_to: (10, 10),
            lengthen: false,
        }
    }

    pub fn try_eat(&mut self){
        self.head_to = self.body[0];
    }

    pub fn tick(&mut self){
        let head = self.body[0];
        let tail = self.body.last().unwrap().clone();
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
        
        if self.lengthen {
            self.body.push(tail);
            self.lengthen = false;
        }
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
