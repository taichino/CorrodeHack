// CorrodeHack - A NetHack-inspired roguelike in Rust

// `use` imports types from crates/modules, like Swift's `import`.
// `crossterm` is an external crate for terminal manipulation.
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout};

enum Tile {
    Floor,
    Wall,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    row.push(Tile::Wall);
                } else {
                    row.push(Tile::Floor);
                }
            }
            tiles.push(row);
        }
        Self { tiles, width, height }
    }

    fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.width > x && self.height > y && matches!(self.tiles[y][x], Tile::Floor)
    }

    fn to_char(tile: &Tile) -> char {
        match tile {
            Tile::Wall => '#',
            Tile::Floor => '.',
        }
    }
}

// A new struct that holds the entire game state.
// Notice `player_x`/`player_y` are separate from the Map —
// the player isn't a tile, they're *on* a tile.
struct Game {
    map: Map,
    player_x: usize,
    player_y: usize,
}

impl Game {
    fn new() -> Self {
        let map = Map::new(20, 10);
        Self {
            map,
            player_x: 10,
            player_y: 5,
        }
    }

    // `&self` — immutable borrow. This method can READ game state but not change it.
    // Like a Swift method on a struct without `mutating`.
    fn display(&self) {
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == self.player_x && y == self.player_y {
                    print!("@");
                } else {
                    print!("{}", Map::to_char(tile));
                }
            }
            print!("\r\n");
        }
    }

    fn move_player(&mut self, key: KeyCode) {
        let (new_x, new_y) = match key {
            KeyCode::Char('h') | KeyCode::Left => (self.player_x - 1, self.player_y),
            KeyCode::Char('j') | KeyCode::Down => (self.player_x, self.player_y + 1),
            KeyCode::Char('k') | KeyCode::Up => (self.player_x, self.player_y - 1),
            KeyCode::Char('l') | KeyCode::Right => (self.player_x + 1, self.player_y),
            _ => return,
        };

        if self.map.is_walkable(new_x, new_y) {
            self.player_x = new_x;
            self.player_y = new_y;
        }
    }
}

fn main() -> io::Result<()> {
    // Enable "raw mode" — keypresses are sent immediately without Enter.
    // Like setting the terminal to non-canonical mode in C.
    terminal::enable_raw_mode()?;

    let mut game = Game::new();

    // `loop` is Rust's infinite loop — like `while true` but cleaner.
    // The compiler knows it never exits naturally, which helps with type checking.
    loop {
        // Clear screen and move cursor to top-left before each frame
        stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

        game.display();

        // `event::read()` returns a `Result` — Rust's way of handling errors.
        // The `?` operator unwraps the Ok value or returns the error early,
        // similar to Swift's `try` but without do/catch blocks.
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                code => game.move_player(code),
            }
        }
    }

    // Clean up: restore normal terminal mode before exiting.
    terminal::disable_raw_mode()?;
    Ok(())
}
