pub struct Game {
    cells: Vec<Vec<Cell>>,
    size: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Dead,
    Live,
}

pub struct Position {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn tick(self: Cell, neighbors: u8) -> Cell {
        match (self, neighbors) {
            (Cell::Live, 2) | (Cell::Live, 3) | (Cell::Dead, 3) => Cell::Live,
            _ => Cell::Dead,
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

pub struct Size(pub usize);

impl Game {
    pub fn new(size: Size) -> Game {
        Game {
            cells: vec![vec![Cell::Dead; size.0]; size.0],
            size: size.0,
        }
    }

    pub fn get(&self, p: Position) -> &Cell {
        &self.cells[p.x][p.y]
    }

    pub fn set_live(&mut self, p: Position) {
        self.cells[p.x][p.y] = Cell::Live
    }

    fn get_range(i: usize, bound: usize) -> (usize, usize) {
        let i_from = if i > 0 { i - 1 } else { i };
        let i_to = if i < bound - 1 { i + 1 } else { i };
        (i_from, i_to)
    }

    fn count_neighbors(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> u8 {
        let mut counter = 0;

        let (x_from, x_to) = Game::get_range(x, grid.len());
        let (y_from, y_to) = Game::get_range(y, grid.len());

        for xx in x_from..=x_to {
            for yy in y_from..=y_to {
                if grid[xx][yy] == Cell::Live && (xx != x || yy != y) {
                    counter += 1
                }
            }
        }
        counter
    }

    pub fn tick(&mut self) {
        let mut next_grid = vec![vec![Cell::Dead; self.size]; self.size];
        for x in 0..self.size {
            for y in 0..self.size {
                let cell = self.cells[x][y];
                let neighbors = Game::count_neighbors(&self.cells, x, y);
                next_grid[x][y] = cell.tick(neighbors);
            }
        }
        self.cells = next_grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_new_game_size() {
        let game = Game::new(Size(100));
        assert_eq!(*game.get(Position::new(99, 99)), Cell::Dead);
    }

    #[test]
    fn set_live_cell() {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(50, 50));
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Live);
    }

    #[test]
    fn any_live_cell_with_fewer_than_two_live_neighbours_dies_as_if_caused_by_under_population() {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(50, 50));
        game.tick();
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Dead);
    }

    #[test]
    fn any_live_cell_with_two_live_neighbours_lives_on_to_the_next_generation() {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(50, 50));
        game.set_live(Position::new(49, 50));
        game.set_live(Position::new(51, 50));
        game.tick();
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Live);
    }

    #[test]
    fn any_live_cell_with_three_live_neighbours_lives_on_to_the_next_generation() {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(50, 50));
        game.set_live(Position::new(49, 50));
        game.set_live(Position::new(51, 50));
        game.set_live(Position::new(50, 51));
        game.tick();
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Live);
    }

    #[test]
    fn any_live_cell_with_more_than_three_live_neighbours_dies_as_if_by_overcrowding() {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(50, 50));
        game.set_live(Position::new(49, 50));
        game.set_live(Position::new(50, 49));
        game.set_live(Position::new(51, 50));
        game.set_live(Position::new(50, 51));
        game.tick();
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Dead);
    }

    #[test]
    fn any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell_as_if_by_reproduction()
    {
        let mut game = Game::new(Size(100));
        game.set_live(Position::new(49, 50));
        game.set_live(Position::new(51, 50));
        game.set_live(Position::new(50, 51));
        game.tick();
        assert_eq!(*game.get(Position::new(50, 50)), Cell::Live);
    }

    #[test]
    fn real_case_scenario() {
        let mut rng = rand::rng();

        let mut game = Game::new(Size(100));
        for _ in 0..50 {
            let p = Position::new(rng.random_range(0..100), rng.random_range(0..100));
            game.set_live(p);
        }
        for _ in 0..5 {
            game.tick();
        }
    }
}
