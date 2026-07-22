# Mamoru 

Mamoru acts as a `commit-msg` hook that embeds a compiled dictionary of **over 106,000 words** to instantly catch and block typos before they enter your version control.

[![Crates.io](https://img.shields.io/crates/v/mamoru.svg)](https://crates.io/crates/mamoru)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/mamoru.svg)](https://crates.io/crates/mamoru)
[![Crates.io](https://img.shields.io/crates/l/mamoru)](https://github.com/Isvane/mamoru/blob/main/LICENSE)

---

## Installation

```console
cargo install mamoru
```

---

## Usage

Navigate to any Git repository and initialize the hook. This automatically configures Mamoru inside `.git/hooks/commit-msg` with the necessary executable permissions:

```console
mamoru init

# If existing git hooks exist, use --force to overwrite it
mamoru init --force
```

Once installed, Mamoru intercepts your git commit commands automatically. If a commit contains unrecognized words, the hook safely aborts the commit and suggests corrections:

```ignore
$ git commit -m "feat: implment algorimth update"

Commit blocked! Typos found in commit message:

  • "implment" -> Did you mean: implement?
  • "algorimth" -> Did you mean: algorithm?

error: Please fix the spelling errors above.
```

### Handling False Positives

If Mamoru flags a valid word (like a company name, internal jargon, or a niche acronym), you can ignore it by creating a `.mamoruignore` file in the root of your repository. Simply add one word per line:

```ignore
mycompanyname
mycustomacronym
```

### Bypassing the Hook

If you are in a rush and need to completely bypass the spellchecker for a single commit (without skipping all of your other git hooks via --no-verify), you can use the MAMORU_SKIP environment variable:

```console
MAMORU_SKIP=1 git commit -m "feat: fix bugz XYZ"
```

### Unintall

To remove mamoru:

```console
mamoru uninstall
```

---

## 🎈 Performance

Benchmarks gathered using `hyperfine` (testing full dictionary load and typo analysis).
```ignore
$ hyperfine -N -i --warmup 10 --min-runs 10000 "./target/release/mamoru check test_commit.txt"

Time (mean ± σ):       2.5 ms ±   0.3 ms
Range (min … max):     2.2 ms …   5.8 ms
```

When no typo were found.
```ignore
Time (mean ± σ):     534.6 µs ±  85.2 µs
Range (min … max):   439.1 µs … 1279.7 µs
```

---

## Acknowledgments

Mamoru's built-in dictionary is compiled from a blend of standard natural language and specialized developer jargon, heavily drawing from:
* **Linux system wordlists** (via `/usr/share/dict`) for robust, everyday English.
* **[cspell](https://github.com/streetsidesoftware/cspell)** repositories for modern programming, DevOps, and open-source.

---

## License

This project is licensed under the [MIT license.](LICENSE)
