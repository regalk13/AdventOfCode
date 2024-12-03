{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
  flake-utils.lib.eachSystem
    [ "x86_64-linux" "aarch64-linux" ]
    (system:
    let
      overlays = [ (import rust-overlay)  ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override{
        extensions = [ "rust-src" "rust-analyzer" ];
      });

      appNativeBuildInputs = with pkgs; [
          # required for the packet inspector on nix
          pkg-config
      ];
      appBuildInputs = with pkgs; [
          rust
          # dependencies for the packet inspector
      ];
      rustSrcPath = "${rust}/lib/rustlib/src/rust/library";
    in 
    rec
    {
        devShell = pkgs.mkShell {
            nativeBuildInputs = appNativeBuildInputs;
            buildInputs = appBuildInputs;    
            shellHook = ''
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appBuildInputs}"
                export RUST_SRC_PATH="${rustSrcPath}"
            '';
        };
    });
}