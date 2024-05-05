{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell rec {
  buildInputs = [
    cargo-watch
  ];
}
