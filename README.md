# mamoru

Mamoru acts as a `commit-msg` hook that embeds a compiled dictionary to instantly catch and block typos before they enter your version control.

[![Crates.io](https://img.shields.io/crates/v/mamoru.svg)](https://crates.io/crates/mamoru)
[![Crates.io](https://img.shields.io/crates/l/mamoru)](https://github.com/Isvane/mamoru/blob/main/LICENSE)

---

## Installation

Install the binary using Cargo:
```bash
cargo install mamoru
```

---

## Usage

Navigate to any Git repository and initialize the hook. This automatically configures Mamoru inside .git/hooks/commit-msg with the necessary executable permissions.
```bash
mamoru --init

# If existing git hooks exist, use --force to overwrite it
mamoru --init --force
```

Once installed, Mamoru intercepts your git commit commands automatically. If a commit contains unrecognized words, the hook safely aborts the commit and suggests corrections.
```bash
$ git commit -m "feat: implment algorimth update"

Commit blocked! Typos found in commit message:

  • "implment" -> Did you mean: implement?
  • "algorimth" -> Did you mean: algorithm?

error: Please fix the spelling errors above.
```

---

## Acknowledgments

Mamoru's built-in dictionary is compiled from a blend of standard natural language and specialized developer jargon, heavily drawing from:
* **Linux system wordlists** (via `/usr/share/dict`) for robust, everyday English.
* **[cspell](https://github.com/streetsidesoftware/cspell)** repositories for modern programming, DevOps, and open-source software terminology.

---

## License

This project is licensed under the [MIT license.](LICENSE)
