// CorrodeHack - A NetHack-inspired roguelike in Rust

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

// --- Traits ---
// A trait defines shared behavior, like a Swift protocol.
// Any type that implements GameEntity can be positioned on the map and drawn.
trait GameEntity {
    fn pos(&self) -> (usize, usize);
    fn glyph(&self) -> char;
}

// Player is now its own struct instead of bare x/y fields on Game.
struct Player {
    x: usize,
    y: usize,
}

// `impl Trait for Type` — like `extension Player: GameEntity` in Swift.
impl GameEntity for Player {
    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn glyph(&self) -> char {
        '@'
    }
}

// `String` is Rust's owned, heap-allocated string — like Swift's `String`.
// (There's also `&str`, a borrowed string slice — we'll get to that.)
struct Monster {
    x: usize,
    y: usize,
    glyph: char,
    name: String,
}

impl GameEntity for Monster {
    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn glyph(&self) -> char {
        self.glyph
    }
}

struct Game {
    map: Map,
    player: Player,
    // Vec<Monster> — a growable array of Monsters, like Swift's [Monster].
    monsters: Vec<Monster>,
}

impl Game {
    fn new() -> Self {
        let map = Map::new(40, 15);
        let player = Player { x: 10, y: 7 };

        // In NetHack, 'd' = dog, 'k' = kobold, 'r' = rat.
        // `String::from()` converts a string literal (&str) into an owned String.
        // Like Swift's String("literal") — though in Swift this is usually implicit.
        let monsters = vec![
            Monster { x: 5,  y: 3,  glyph: 'd', name: String::from("dog") },
            Monster { x: 15, y: 5,  glyph: 'k', name: String::from("kobold") },
            Monster { x: 30, y: 10, glyph: 'r', name: String::from("rat") },
        ];

        Self { map, player, monsters }
    }

    // Checks if any entity is at position (x, y) and returns its glyph.
    // Returns `Option<char>` — Rust's version of Swift's `Optional`.
    //   Some('x') = a value is present
    //   None      = no value (like Swift's nil)
    fn entity_at(&self, x: usize, y: usize) -> Option<char> {
        let (px, py) = self.player.pos();
        if x == px && y == py {
            return Some(self.player.glyph());
        }

        // Iterate over monsters — `&self.monsters` borrows the Vec immutably.
        for monster in &self.monsters {
            let (mx, my) = monster.pos();
            if x == mx && y == my {
                return Some(monster.glyph());
            }
        }

        None
    }

    fn display(&self) {
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                // `if let` unwraps an Option — like `if let glyph = entity_at(...)` in Swift.
                if let Some(glyph) = self.entity_at(x, y) {
                    print!("{}", glyph);
                } else {
                    print!("{}", Map::to_char(tile));
                }
            }
            print!("\r\n");
        }
    }

    fn move_player(&mut self, key: KeyCode) {
        let (new_x, new_y) = match key {
            KeyCode::Char('h') | KeyCode::Left => (self.player.x - 1, self.player.y),
            KeyCode::Char('j') | KeyCode::Down => (self.player.x, self.player.y + 1),
            KeyCode::Char('k') | KeyCode::Up => (self.player.x, self.player.y - 1),
            KeyCode::Char('l') | KeyCode::Right => (self.player.x + 1, self.player.y),
            _ => return,
        };

        if self.map.is_walkable(new_x, new_y) {
            self.player.x = new_x;
            self.player.y = new_y;
        }
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut game = Game::new();

    loop {
        stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

        game.display();

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                code => game.move_player(code),
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
