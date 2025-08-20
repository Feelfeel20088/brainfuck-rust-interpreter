{
  description = "Brainfuck Rust Interpreter dev shell and package";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        brainfuck = pkgs.callPackage ./nix/default.nix { };
      in
      {
        packages = {
          default = brainfuck;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
            pkg-config
          ];

          RUST_SRC_PATH ="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}

