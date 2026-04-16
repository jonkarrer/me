{
  description = "me — project dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Core
            git
            just
            doppler

            # JS runtime & package manager
            bun

            # Python (for asset manipulation)
            uv

            # Deploy
            flyctl

            # Terminal multiplexer
            tmux

            # Add project-specific packages below
          ];

          shellHook = ''
            echo "me Shell"
            export PATH="/usr/bin:/usr/sbin:$PATH"
            export ZSH_SYNTAX_HIGHLIGHTING=${pkgs.zsh-syntax-highlighting}/share/zsh-syntax-highlighting
            export ZSH_AUTOSUGGESTIONS=${pkgs.zsh-autosuggestions}/share/zsh-autosuggestions

            if [ ! "$SHELL" = "$(command -v zsh)" ]; then
              export ZDOTDIR="$HOME/devjon/configs/shells/mini.local"
              exec zsh
            fi
          '';
        };
      });
}
