# pasgen

Hey, Users! 👋

This is a CLI program that generates a random password of a desired length (any value that fits in a `u32`).

## How to use

```sh
pasgen <length>
```

Example:
```sh
pasgen 20
#wIPQJ*BpD1?4]BF/5hjn
```

## How to download / install

### Option 1: Install via Cargo (requires Rust)
```bash
cargo install --git https://github.com/anoninus/pasgen
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```
