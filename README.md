# stromboli

## TODO

- [x] Idempotency
  - [x] Make an `Action` trait (or something) that I can with something like an
        `IdempotentAction`.
    - I.e. I have `GitHubRelease::fetch_url()`, but it'd be nice to,
      in idempotent mode, not do the fetch if we already have the downloaded file.
      Maybe then we have `FetchUrl` that implements `Action` and `ItempotentAction`.
    - `Action`'s return value is just the value, whereas `IdempotentAction` returns a `Success`
      that wraps the value.
- [x] Logging
  - [x] Instead of having methods for `install()` and `install_with_logging()`, I should just depend on
        Rust's `log` stuff (if it's enabled, log things). As such, consider making my own logger that's
        `log`-aware.
  - [x] Add logging for each method that does something.
- [ ] Install methods. Types like `Install<RemoteShellScript>` aren't using `RemoteShellScript`
      methods--fix that.

- Things that don't get installed:
  - homebrew
    - topgrade. And this could get run at the end of the bootstrap process.
    - bat
    - lua-language-server
    - rust-analyzer
    - vale 
  - MacOS
    - Alfred
    - Slack
    - Tutanota Desktop
    - nix
  - cargo-install things:
    - zr?? (is this via homebrew?)
    - wasm-snip
    - wasm-pack
    - stylua
    - shellharden
    - selene
    - jless
    - grcov
    - cargo-watch
    - cargo-outdated
    - cargo-insta
    - cargo-cache
    - cargo-bloat
    - cargo-add
- Problems
  - .tmux.conf points to /usr/local/bin/zsh
  - iTerm color config & font not setup (see yadm docs for hints)
  - my `init.rs` isn't pulled and built
- Things I don't want any more
  - tmuxinator
