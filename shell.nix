let
  nixpkgs = fetchTarball
    "https://github.com/NixOS/nixpkgs/archive/nixos-25.05.tar.gz";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };

in
pkgs.mkShell {
  packages = with pkgs; [ clippy taplo ];
  # nativeBuildInputs = with pkgs; [
  #   cargo
  # ];

  buildInputs = with pkgs; [
    cargo
    rust-analyzer
    rustc
    clippy
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
