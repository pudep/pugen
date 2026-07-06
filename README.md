# Pugen

Pugen is a CLI program that generates a random password of a desired length (any value that fits in a `u32`).

## How to use

```sh
# For 20 digit long password
pas 20
```

Example:
```sh
pas 20
#wIPQJ*BpD1?4]BF/5hjn
```

# How to download / install

```bash
cargo install --git https://github.com/pudep/pasgen

sh -c '
  case "$(basename "$SHELL")" in
    fish)
      rc="$HOME/.config/fish/config.fish"
      mkdir -p "$HOME/.config/fish"
      line="set -gx PATH \$HOME/.cargo/bin \$PATH"
      ;;
    zsh)
      rc="$HOME/.zshrc"
      line="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
      ;;
    *)
      rc="$HOME/.bashrc"
      line="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
      ;;
  esac

  grep -qxF "$line" "$rc" 2>/dev/null || echo "$line" >> "$rc"

  exec $SHELL
'
```
