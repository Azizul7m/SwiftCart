{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
      in {

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            postgresql
            cowsay
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
          env = {
            RUST_LOG = "debug";
            PG_CONFIG = "${pkgs.postgresql}/bin/pg_config";
          };
          shellHook = ''
            cowsay "Welcome to the Rust development shell!"
          '';
        };

      });
}
