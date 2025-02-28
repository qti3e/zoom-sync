{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;

        config.allowUnfree = true;
        config.allowBroken = true;
      };
    in
    {
      packages.${system}.default = (
        pkgs.callPackage (
          { lib, rustPlatform }:
          rustPlatform.buildRustPackage {
            name = "zoom-sync";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            nativeBuildInputs = with pkgs; [
              pkg-config
              addDriverRunpath
            ];
            buildInputs = with pkgs; [
              systemd # for libudev
              openssl # for http request to ipinfo and open-meteo
            ];
            fixupPhase = ''
              addDriverRunpath $out/bin/zoom-sync
            '';
          }
        ) { }
      );
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustup
          rust-analyzer
        ];
        inputsFrom = [ self.packages.${system}.default ];
      };
    };
}
