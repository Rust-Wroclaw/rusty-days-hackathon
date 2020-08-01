//! This contains the Conway's Game of Life logic. Notably:
//! * generating the next generation from the previous one,
//! * producing an iterator over all the organisms in a generation,
//! * counting the cells of an organism, and
//! * determining whether an organism is dying.

mod cell;
mod error;
mod grid;
mod index;
mod organism;

pub use error::GameError;
pub use grid::Grid;
pub use index::Index;

pub type Result<V> = std::result::Result<V, GameError>;

/// A structure holding all the grids, that are being played at the same time
#[derive(Clone)]
pub struct Conway {
    stopped: bool,
    grids: Vec<Grid>,
}

impl Conway {
    pub fn new() -> Self {
        Self {
            grids: Vec::new(),
            stopped: false,
        }
    }

    /// Creates new Conway game with `capacity` number of games. All of the games are being
    /// randomized and started
    pub fn start_with_capacity(capacity: usize) -> Self {
        let mut grids = Vec::<Grid>::with_capacity(capacity);

        for _ in 0..capacity {
            grids.push(Grid::random());
        }

        Self {
            grids,
            stopped: false,
        }
    }

    /// Returns iterator over the games
    pub fn iter(&self) -> std::slice::Iter<Grid> {
        self.grids.iter()
    }

    /// Returns mutable iterator over the games
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Grid> {
        self.grids.iter_mut()
    }

    /// Adds a new grid to the game
    pub fn add_game(&mut self, game: Grid) {
        self.grids.push(game);
    }

    pub fn number_of_games(&self) -> usize {
        self.grids.len()
    }

    /// Removes the grid with the `index`
    // NOTE: It might be a good idea to use some identifiers for the games
    pub fn remove_game(&mut self, index: usize) -> Result<Grid> {
        if index >= self.number_of_games() {
            return Err(GameError::IndexOutOfBounds(index.into()));
        }

        Ok(self.grids.remove(index))
    }

    pub fn next_gen(&mut self) {
        if self.stopped {
            return;
        }

        self.grids.iter_mut().for_each(|grid| {
            grid.next_gen();
        })
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn start(&mut self) {
        self.stopped = false;
    }

    pub fn toggle(&mut self) {
        self.stopped = !self.stopped;
    }

    pub fn stop_game(&mut self, game_index: usize) -> Result<()> {
        self.grids
            .get_mut(game_index)
            .ok_or_else(|| GameError::IndexOutOfBounds(game_index.into()))?
            .stop();
        Ok(())
    }

    pub fn start_game(&mut self, game_index: usize) -> Result<()> {
        self.grids
            .get_mut(game_index)
            .ok_or_else(|| GameError::IndexOutOfBounds(game_index.into()))?
            .start();
        Ok(())
    }

    pub fn toggle_game(&mut self, game_index: usize) -> Result<()> {
        self.grids
            .get_mut(game_index)
            .ok_or_else(|| GameError::IndexOutOfBounds(game_index.into()))?
            .toggle();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_capacity() {
        let mut games = Conway::start_with_capacity(5);
        assert_eq!(games.number_of_games(), 5);

        games.add_game(Grid::random());
        assert_eq!(games.number_of_games(), 6);

        games.add_game(Grid::random());
        assert_eq!(games.number_of_games(), 7);

        games.remove_game(1).unwrap();
        assert_eq!(games.number_of_games(), 6);

        games.remove_game(1).unwrap();
        assert_eq!(games.number_of_games(), 5);
    }

    use std::time::Instant;

    #[test]
    fn test_if_grids_change() {
        let mut games = Conway::start_with_capacity(10);

        let old_games = games.clone();
        let now = Instant::now();
        games.next_gen();
        let now = now.elapsed().as_secs_f32();
        println!("{}", now);
        assert!(!games
            .iter()
            .zip(old_games.iter())
            .any(|(new, old)| old == new));

        let old_games = games.clone();
        games.next_gen();
        assert!(!games
            .iter()
            .zip(old_games.iter())
            .any(|(new, old)| old == new));

        let old_games = games.clone();
        games.next_gen();
        assert!(!games
            .iter()
            .zip(old_games.iter())
            .any(|(new, old)| old == new));
    }
}
