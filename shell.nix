{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = with pkgs; [
    sqlite
    cargo
    libclang
    clang
    rustc
    rust-analyzer

    pkg-config
    sqlx-cli
    openssl
    openssl.dev
    libffi
    libffi.dev
    llvmPackages.libclang
  ];
}