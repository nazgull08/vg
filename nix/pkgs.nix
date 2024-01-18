# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "377dcbdb1186c3bebbac5837f69aaff6e04ee582";
  sha256  = "sha256:124qls5n21dqvfj8g8givwg7aa7lsbs2i74mdwsxgbf17413bdmg";
})
