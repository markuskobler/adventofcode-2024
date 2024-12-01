{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
        in
        with pkgs;
        {
          devShells.default = mkShell {
            buildInputs = [
              samply
            ] ++ [
              # MacOS tooling
              (with darwin.apple_sdk.frameworks; lib.optionals stdenv.isDarwin [
                Security
                SystemConfiguration
              ])
            ];
          };
        }
      );
}
