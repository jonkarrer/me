# Using Nix in the AI Era

Every project I touch now has a `flake.nix` at its root. The only thing I install on a new machine is Nix itself. Everything else — Bun, cargo, xcodegen, flyctl, doppler, a specific Python — lives in a flake that `nix develop` drops me into.

The habit started as a reaction to my own laptop becoming a museum of half-broken tool versions. It's held up even better as AI coding agents have joined the loop.

## The setup

All my projects live under `~/devjon/projects`, and every one has its own `flake.nix` describing the exact shell it needs. A small web project looks like this:

```nix
devShells.default = pkgs.mkShell {
  buildInputs = with pkgs; [
    git just doppler
    bun
    uv
    flyctl
    tmux
  ];
};
```

An iOS + web hybrid pulls in `xcodegen`, `cocoapods`, `libimobiledevice`. A Rust + onchain project pulls in `foundry` via an overlay. Each shell is just a list, and I can read exactly what the project needs without guessing.

My zsh config lives in `~/devjon/configs/shells`, and every flake's `shellHook` re-execs zsh pointing at one of those `ZDOTDIR`s:

```nix
shellHook = ''
  if [ ! "$SHELL" = "$(command -v zsh)" ]; then
    export ZDOTDIR="$HOME/devjon/configs/shells/mini.local"
    exec zsh
  fi
'';
```

Every project gets the same prompt, keybindings, and syntax highlighting — without any of that living in the project's own flake.

## Why this matters more now

A couple of years ago, pinned tool versions were a nice-to-have. With agents writing code across many projects, it's a different shape of important.

- **Agents run tools, not just edits.** A coding agent calling `bun run build` or `cargo test` needs the right versions available. A flake guarantees that, regardless of the machine — or container — the agent is running in.
- **Agents aren't great at reading install instructions.** A README that says "install Xcode Command Line Tools, brew install libimobiledevice, nvm use 22, pip install uv" is a gauntlet. A `nix develop` entrypoint is a single instruction the agent (and I) can always take.
- **Divergence is expensive.** If agent-authored code assumes Node 22 but my machine has 18, the bug lives in two places. Pinning the shell means the agent and I are always looking at the same world.

## What I still want to fix

A few rough edges. Darwin evaluation is slow on the first run — a first `nix develop` on a fresh machine can take a while. Flake locks drift across projects unless I remember to refresh them. And each project's `flake.nix` has enough repetition that a shared `mkShell` helper would pay for itself.

But on the whole: Nix has done more to quiet my setup than any other tool. I type `nix develop`, the shell spins up, and I stop thinking about toolchain.
