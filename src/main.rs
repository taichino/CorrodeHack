// CorrodeHack - A NetHack-inspired roguelike in Rust

// Enums in Rust: like Swift enums, each variant is a distinct value.
// Unlike C enums (just integers), Rust enums are full types the compiler tracks.
enum Tile {
    Floor,
    Wall,
    Player,
}

// Structs in Rust: similar to Swift structs — value types with stored properties.
// `width` and `height` are `usize` — an unsigned integer sized for indexing
// (like Swift's `Int` but unsigned, similar to `size_t` in C).
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

// `impl` adds methods to a struct, like Swift extensions.
// `&self` is like Swift's `self` — an immutable reference to the instance.
impl Map {
    // `fn new(...)` is a constructor by convention (Rust has no `init`).
    // It returns `Self` (the type being impl'd), like a Swift static factory method.
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

    // TODO(human): Implement the `to_char` method.
    // Given a reference to a Tile, return the ASCII character to display.
    // Use `match` (Rust's version of Swift's `switch`).
    //
    // NetHack traditionally uses:
    //   Wall  => '#'
    //   Floor => '.'
    //   Player => '@'
    //
    // Hint: the syntax looks like this:
    //   fn to_char(tile: &Tile) -> char {
    //       match tile {
    //           Tile::Variant => 'x',
    //           ...
    //       }
    //   }
    fn to_char(tile: &Tile) -> char {
        match tile {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Player => '@',
        }
    }

    fn display(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{}", Self::to_char(tile));
            }
            println!();
        }
    }
}

fn main() {
    let mut map = Map::new(20, 10);

    // Place the player near the center
    map.tiles[5][10] = Tile::Player;

    map.display();
}
