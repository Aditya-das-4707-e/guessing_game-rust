# Guessing Game — Rust

A simple command-line number guessing game written in Rust.
The program picks a random secret number between 1 and 100, then asks you to guess until you get it right.

This README explains the full code **line by line**, **what is imported and why**, and **how to add imports yourself**.

---

## 1. Project Structure

```text
guessing_game/
├── Cargo.toml   # project metadata + dependencies (what external crates to import)
├── Cargo.lock   # locked exact versions (auto-generated)
└── src/
    └── main.rs  # all game code
```

## 2. Requirements

- Rust + Cargo installed. Check with:

```bash
rustc --version
cargo --version
```

If not installed, install from <https://rustup.rs/>:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 3. How to Run

```bash
cd guessing_game
cargo run
```

Other useful commands:

```bash
cargo build          # compile to ./target/debug/guessing_game
cargo build --release # optimized build to ./target/release/guessing_game
cargo check          # fast type-check without building
cargo update         # update dependencies in Cargo.lock
```

Example session:

```text
Guess the number
50
Too small!
hello
Please input a number!
75
Too big!
63
You win!
```

---

## 4. How Imports Work in Rust (How to Import)

Rust has two steps for imports:

### Step A: Declare the dependency in `Cargo.toml` (only for external crates)

Standard library (`std`) is built-in — no entry needed.

External crates (like `rand`) must be listed under `[dependencies]`:

```toml
[dependencies]
rand = "0.10.3"
```

Meaning: download crate named `rand`, version compatible with `0.10.3` (`^0.10.3`).

How to add it yourself (any of these):

```bash
cargo add rand
cargo add rand@0.10.3
```

Or manually edit `Cargo.toml` then run:

```bash
cargo build
# cargo downloads + compiles rand automatically from crates.io
```

### Step B: Bring items into scope with `use` in `main.rs`

```rust
use std::io;            // built-in, no Cargo.toml entry needed
use std::cmp::Ordering; // built-in, no Cargo.toml entry needed
use rand::RngExt;       // external, requires rand in Cargo.toml
```

Rules:

1. `use` goes at the top of the file.
2. `std::...` = Rust standard library, always available.
3. `rand::...` = external crate, only works if `Cargo.toml` has it.
4. You can import a module (`use std::io;` then call `io::stdin()`), a type/enum (`use std::cmp::Ordering;`), or a trait (`use rand::RngExt;`).
5. Full path without `use` is also valid, e.g. `std::io::stdin()` works even without `use std::io;` — `use` is just a shortcut.

---

## 5. What I Import and Why

| Import | Type | Built-in or External? | Why it is needed? |
|---|---|---|---|
| `use std::io;` | module | Built-in (`std`) | To read keyboard input via `io::stdin().read_line(&mut guess)`. Without it you cannot take user input. |
| `use std::cmp::Ordering;` | enum | Built-in (`std`) | `guess.cmp(&secret_number)` returns `Ordering::Less / Greater / Equal`. Needed for the `match` comparison. Without it you would have to write `std::cmp::Ordering::Less` every time. |
| `use rand::RngExt;` | trait | External (`rand = "0.10.3"`) | Provides the method `random_range()`. In `rand 0.9+`, random methods live in the `RngExt` trait. If you omit this `use`, the compiler error is `no method named random_range found`. Older tutorials use `use rand::Rng;` — that was for `rand 0.8` with `gen_range()`. |

Why `RngExt` and not just `rand`?

- `rand::rng()` returns a random number generator object.
- That object alone has no `random_range()` method until you import the trait that defines it.
- Traits = shared behaviour. Importing `RngExt` “unlocks” `.random_range(1..101)` on that generator.

---

## 6. Full Code (`src/main.rs`)

```rust
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::rng().random_range(1..101);
    // println!("The secret number is: {}", secret_number);

    // println!("Please input your guess : ");
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("code run failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        // println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## 7. Line-by-Line Explanation

### `Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
```

- `[package]`: section for project metadata.
- `name = "guessing_game"`: crate/executable name.
- `version = "0.1.0"`: your program version.
- `edition = "2024"`: which Rust language edition/rules to use (2024 = latest).

```toml
[dependencies]
rand = "0.10.3"
```

- `[dependencies]`: list of external crates from <https://crates.io>.
- `rand = "0.10.3"`: the only external import. Needed for random numbers. `std` has no random generator, so we need this crate.

### `src/main.rs`

**Line 1: `use rand::RngExt;`**

- Imports the `RngExt` trait from the external `rand` crate.
- Why: unlocks `.random_range()` method used on line 7.
- How: works only because `Cargo.toml` contains `rand = "0.10.3"`. Remove that line and this `use` fails to compile.

**Line 2: `use std::cmp::Ordering;`**

- Imports the `Ordering` enum (`Less`, `Greater`, `Equal`) from standard library.
- Why: used in `match` on lines 26–31 to compare guess vs secret.
- How: `std` needs no `Cargo.toml` entry. Alternative without import: write `std::cmp::Ordering::Less` fully.

**Line 3: `use std::io;`**

- Imports the `io` module for input/output.
- Why: used on line 13 as `io::stdin()` to read keyboard input.
- How: `std` needs no `Cargo.toml` entry.

**Line 4: (blank)**

- Readability only.

**Line 5: `fn main() {`**

- Program entry point. Every Rust executable starts here. `fn` defines a function, `main` with no args and no return.

**Line 6: `println!("Guess the number");`**

- Macro (`!` = macro) that prints text + newline to terminal.
- Shows the game title.

**Line 7: `let secret_number = rand::rng().random_range(1..101);`**

- `rand::rng()`: creates a thread-local random number generator (seeded by OS).
- `.random_range(1..101)`: trait method from `RngExt`; picks random integer in range `1..101` = 1 to 100 inclusive of 1, exclusive of 101.
- `let secret_number`: immutable binding; stores the number to guess. Type inferred as `u32` because it is later compared with `guess: u32` via `guess.cmp(&secret_number)`.
- This runs once, before the loop, so the secret stays constant.

**Line 8: `// println!("The secret number is: {}", secret_number);`**

- Commented-out debug line. Would reveal the answer. Useful while testing. `//` = single-line comment, `{}` = placeholder filled by `secret_number`.

**Line 10: `// println!("Please input your guess : ");`**

- Commented-out prompt. Can be uncommented to show a prompt each game start.

**Line 11: `loop {`**

- Infinite loop. Repeats guessing until explicit `break` on line 30. Needed because user may need multiple tries.

**Line 12: `let mut guess: String = String::new();`**

- `let mut`: mutable variable (can be changed).
- `guess: String`: type annotation — growable UTF-8 string.
- `String::new()`: creates empty string. `::` calls associated function `new` on type `String`.
- Why inside loop: fresh empty buffer every iteration, otherwise old input would accumulate.
- Why `mut`: `read_line` appends into it, so it must be mutable.

**Line 13: `io::stdin().read_line(&mut guess).expect("code run failed");`**

- `io::stdin()`: handle to standard input (keyboard).
- `.read_line(&mut guess)`: reads one line (until Enter) and appends to `guess`. Takes `&mut` (mutable reference/borrow) so it can modify `guess` without taking ownership.
- Returns `Result<usize>` (`Ok(bytes_read)` or `Err(error)`).
- `.expect("code run failed")`: if `Err`, crash with that message; if `Ok`, continue. Simple error handling for beginners.

**Lines 15–21: `let guess: u32 = match guess.trim().parse() { ... };`**

- This is *shadowing*: reuses name `guess` but with new type `u32` (unsigned 32-bit int 0..4,294,967,295). Old `String` is shadowed.
- `guess.trim()`: removes whitespace/newline (`"42\n"` → `"42"`). `read_line` includes Enter key as `\n`/`\r\n`, must trim.
- `.parse()`: converts `&str` to a number. Type inferred from `Ok(num)` → `u32` binding and the later `guess.cmp(&secret_number)` comparison. Returns `Result<u32, _>` (`Ok(number)` on success, `Err(error)` on failure).
- `match ... { Ok(num) => num, Err(_) => { ... } }`: handles both cases instead of crashing:
  - `Ok(num) => num`: if parse succeeded, unwrap and store the number in new `guess`.
  - `Err(_) =>`: if parse failed (e.g. user typed `hello` or empty line), `_` means “ignore the exact error”. Inside the block:
    - `println!("Please input a number!");` warns the user.
    - `continue;` skips the rest of the loop iteration (skips the `cmp`/`match` below) and jumps to the next `loop` round to ask again.
- **Why this is better than `.expect(...)`:** old code `guess.trim().parse().expect(...)` crashed the whole program on bad input. This version keeps the game alive.

**Line 23: `// println!("You guessed {guess}");`**

- Commented-out echo. Would confirm what was parsed. `{guess}` = inline variable interpolation (Rust 2021+).

**Line 25: `match guess.cmp(&secret_number) {`**

- `guess.cmp(&secret_number)`: compares two numbers, returns `Ordering` enum.
- `&secret_number`: passes by reference (borrow) as `cmp` expects `&Self`.
- `match`: exhaustive pattern matching — must handle all three `Ordering` cases below.

**Line 26: `Ordering::Less => println!("Too small!"),`**

- Arm 1: if `guess < secret`, print hint. `=>` separates pattern from code. Trailing `,` ends arm.

**Line 27: `Ordering::Greater => println!("Too big!"),`**

- Arm 2: if `guess > secret`, print hint.

**Lines 28–31:**

```rust
Ordering::Equal => {
    println!("You win!");
    break;
}
```

- Arm 3: if equal, print win message and `break` out of `loop`, ending the program. Braces `{}` needed because arm has two statements.

**Line 32: `}`**

- Closes `match`.

**Line 33: `}`**

- Closes `loop`.

**Line 34: `}`**

- Closes `fn main`.

---

## 8. Common Changes / Experiments

1. Show prompt each turn — add inside `loop` before `read_line`:
   ```rust
   println!("Please input your guess:");
   ```
2. ✅ Already done in this version: bad input no longer crashes. Old code was `guess.trim().parse().expect(...)` which crashed on `abc`. New code uses `match + continue` to print `Please input a number!` and ask again.
3. Limit range message — change line 6 to:
   ```rust
   println!("Guess a number between 1 and 100");
   ```
4. Count attempts — add `let mut tries = 0;` before `loop`, add `tries += 1;` inside `loop`, and print it on win:
   ```rust
   println!("You win in {tries} tries!");
   ```

---

## 9. Troubleshooting Imports

| Error | Cause | Fix |
|---|---|---|
| `unresolved import rand` / `can't find crate for rand` | Missing `Cargo.toml` entry | Add `rand = "0.10.3"` under `[dependencies]`, run `cargo build` |
| `no method named random_range found` | Missing trait import | Add `use rand::RngExt;` at top |
| `no method named gen_range found` | Old tutorial + new `rand` | Use `random_range` + `RngExt`, not `gen_range` + `Rng` |
| `cannot find value Ordering in this scope` | Missing import | Add `use std::cmp::Ordering;` |
| `cannot find function stdin in module io` | Wrong path | Use `io::stdin()` with `use std::io;`, not `use std::io::stdin;` + `stdin()` confusion |

---

## 10. References

- Rust Book Ch.2 (Guessing Game Tutorial): <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>
- `rand` crate docs: <https://docs.rs/rand>
- `std::io::Stdin`: <https://doc.rust-lang.org/std/io/struct.Stdin.html>
- `std::cmp::Ordering`: <https://doc.rust-lang.org/std/cmp/enum.Ordering.html>
