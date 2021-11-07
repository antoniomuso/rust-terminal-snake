use std::{arch::x86_64::{_rdrand16_step}, collections::LinkedList, fmt::{Display, Error, write}, io::Empty, ops::{Add, AddAssign}, u16::MAX};

#[derive(Clone)]
pub enum Cell {
    Empty,
    Food,
    Snake,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "🟦"),
            Cell::Food => write!(f, "🟤"),
            Cell::Snake => write!(f, "🟩"),
        }
    }
}

macro_rules! set{
    ($self:ident, $a:expr,$b:expr,$c:expr) => {
        $self.cells[($a * $self.width as i32 + $b) as usize] = $c;
    };
}

#[derive(Default)]
pub struct GameStatus {
    cells: Vec<Cell>,
    pub snake: Snake,
    foods: Vec<point>,
    pub height: u32,
    pub width: u32,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq,)]
pub struct point(pub i32, pub i32);

#[derive(Debug, Default)]
pub struct Snake {
    head: point,
    tail: LinkedList<point>,
}

impl Add for point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        point (
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

impl AddAssign for point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Display for point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}


impl Snake {
    pub fn new(head: (i32, i32)) -> Snake {
        let mut tail: LinkedList<point> = LinkedList::new();
        tail.push_back(point (head.0, head.1));

        Snake {
            head: point(head.0, head.1),
            tail,
        }
    }

    pub fn get_tail_len(&self) -> usize {
        self.tail.len()
    }

    pub fn get_head(&self) -> point {
        self.head
    }

    pub fn get_tail(&self) -> &LinkedList<point> {
        return &self.tail;
    }

    pub fn move_snake(&mut self, direction: point) -> bool {

        let old_head = self.head;
        self.head += direction;

        if self.tail.is_empty() {
            return true;
        }

        self.tail.pop_front();
        self.tail.push_back(old_head);

        !self.is_self_eating()
    }

    pub fn is_self_eating(&self) -> bool {
        for p in self.tail.iter() {
            if *p == self.head {
                return true;
            }
        }
        false
    }
}


impl GameStatus {
    pub fn new(height: u32, width: u32) -> GameStatus {
        let matrix_size = (height * width) as usize;
        GameStatus {
            snake: Snake::new((0,0)),
            cells: vec![Cell::Empty; matrix_size],
            foods: vec![],
            height,
            width
        }
    }

    pub fn is_gameover(&self) -> bool {
        let head = self.snake.head;
        self.snake.is_self_eating() || head.0 >= self.width as i32 || head.1 >= self.height as i32 || head.0 < 0 || head.1 < 0
    }

    pub fn eat(&mut self) {
        let head = self.snake.get_head();
        if let Some(i) = self.foods.iter().position(|p| *p == head) {
            self.foods.remove(i);
            self.snake.tail.push_back(head);
        }
    }

    pub fn random() -> u16 {
        let mut rand = 0;
        
        unsafe {
            while _rdrand16_step(&mut rand) == 0 {}
        }
        rand
    }

    pub fn update_matrix (&mut self) {
        for i in 0..self.width as i32 {
            for j in 0..self.height as i32 {
                self.set_cell(point(i , j), self::Cell::Empty);
            }
        }
        for p in self.foods.iter() {
            set!(self, p.0, p.1, Cell::Food);
        }
        for p in self.snake.tail.iter() {
            set!(self, p.0, p.1, Cell::Snake);
        }
        let head = &self.snake.head;
        set!(self, head.0, head.1, Cell::Snake);
    }

    pub fn get_cell(&self, point(x,y): point) -> Result<Cell, Error> {
        if x >= self.width as i32 || x < 0 || y >= self.height as i32 || y < 0 {
            return Result::Err(Error);
        }

        Ok(self.cells[(x * self.width as i32 + y) as usize].clone())
    }

    pub fn set_cell(&mut self, point(x,y): point, cell: Cell) -> Result<(), Error> {
        if x >= self.width as i32 || x < 0 || y >= self.height as i32 || y < 0 {
            return Result::Err(Error);
        }

        set!(self, x, y, cell);
        Ok(())
    }

    pub fn generate_food_if_empty (&mut self) {
        if self.foods.is_empty() {
            for _ in 0..10 {
                self.generate_food();
            }
        }
    }

    pub fn generate_food(&mut self) {
        let rand_x = Self::random();
        let rand_y = Self::random();
        
        let rand_x = ((rand_x as f32 / MAX as f32) * self.width as f32) as i32;
        let rand_y = ((rand_y as f32 / MAX as f32) * self.height as f32) as i32;

        if let Some(_) = self.foods.iter().position(|point (x, y)| *x == rand_x && *y == rand_y) {
            return self.generate_food();
        }
        self.foods.push(point (rand_x, rand_y));
    }

}
