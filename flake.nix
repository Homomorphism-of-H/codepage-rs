{
  description = "A flake providing a devShell for work with simple rust projects.";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      rust-overlay,
      nixpkgs,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "i686-linux"
      ];
      eachSystem = nixpkgs.lib.genAttrs systems;

      overlays = [ (import rust-overlay) ];
    in
    {
      devShells = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          runtimeDeps = [ ];
          buildDeps = with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ];
          devDeps = [ ];
        in
        {
          default = pkgs.mkShell {
            shellHook = ''
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps ++ devDeps ++ [ pkgs.rust-bin.beta.latest.default ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath runtimeDeps}";
          };
        }
      );
    };
}
