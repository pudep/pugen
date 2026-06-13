# pasgen

Hey, Users! 👋

This is a CLI program that generates a random password of a desired length (any value that fits in a `u32`).

## How to use

```sh
pasgen <length>
```

Example:
```sh
#command -->
pasgen 20
#output --> 
wIPQJ*BpD1?4]BF/5hjn
```

## How to download / install

### Option 1: Install via Cargo (requires Rust)
```bash
cargo install --git https://github.com/anoninus/pasgen
```

### Option 2: Build from source
```bash
git clone https://github.com/anoninus/pasgen.git
cd pasgen
cargo build --release
./target/release/pasgen 20
```

### Option 3: Pre-built binary
Check the [Releases](https://github.com/anoninus/pasgen/releases) page for downloadable binaries.

