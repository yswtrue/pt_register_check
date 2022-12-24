{
  description = "A basic flake with a shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "pt_register_check"; # make this what ever your cargo.toml package.name is
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ]; # just for the host building the package
          buildInputs = [ pkgs.openssl ]; # packages needed by the consumer
        };
        dockerImage = pkgs.dockerTools.buildImage {
          name = "rust-nix-blog";
          config = { Cmd = [ "${myRustBuild}/bin/pt_register_check" ]; };
        };
      in
      with pkgs;
      {
        packages = {
          rustPackage = myRustBuild;
          docker = dockerImage;
        };

        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            exa
            fd
            rust-bin.stable.latest.complete
            rust-analyzer
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
        };
      }
    );
}
