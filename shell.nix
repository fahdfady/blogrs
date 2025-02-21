{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = with pkgs; [
    sqlite
    cargo
    libclang
    clang
    rustc
    rust-analyzer

    openssl
    openssl.dev
    libffi
    libffi.dev
    llvmPackages.libclang
  ];
}