let
  nixpkgs = fetchTarball
    "https://github.com/NixOS/nixpkgs/archive/nixos-25.11.tar.gz";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };

in
pkgs.mkShell {
  packages = with pkgs; [ clippy taplo ];
  buildInputs = with pkgs; [
    cargo
    rust-analyzer
    rustc
    clippy
    prettierd
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
