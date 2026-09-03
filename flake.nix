{
  description = "me — project dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell-lib.url = "path:/Users/jokar/devjon/configs/shells/shared";
  };

  outputs = { self, nixpkgs, flake-utils, devshell-lib }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = devshell-lib.mkDevShell {
          inherit pkgs;
          name = "me";
          extraPackages = with pkgs; [
            # JS runtime & package manager
            bun

            # Deploy
            flyctl

            # Add project-specific packages below
          ];
        };
      });
}
