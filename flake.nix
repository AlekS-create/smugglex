{
  description = "A powerful HTTP Request Smuggling testing tool written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        packages = {
          smugglex = rustPlatform.buildRustPackage {
            pname = "smugglex";
            version = "0.0.1";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            meta = with pkgs.lib; {
              description = "A powerful HTTP Request Smuggling testing tool written in Rust";
              homepage = "https://github.com/hahwul/smugglex";
              license = licenses.mit;
              maintainers = [ ];
              mainProgram = "smugglex";
            };
          };

          default = self.packages.${system}.smugglex;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            cargo-watch
            rust-analyzer
          ];

          shellHook = ''
            echo "smugglex development environment"
            echo "Run 'cargo build' to build the project"
            echo "Run 'cargo run' to run the project"
          '';
        };

        apps = {
          smugglex = flake-utils.lib.mkApp {
            drv = self.packages.${system}.smugglex;
          };
          default = self.apps.${system}.smugglex;
        };
      }
    );
}
