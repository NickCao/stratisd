{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = import nixpkgs { inherit system; }; in
      with pkgs; rec {
        devShell = mkShell {
          nativeBuildInputs = with rustPlatform; [
            bindgenHook
            rust.cargo
            rust.rustc
            rust-analyzer
            rustfmt
            clippy
            pkg-config
            asciidoc
          ];
          buildInputs = [
            dbus
            cryptsetup
            util-linux
            udev
          ];
          EXECUTABLES_PATHS = lib.makeBinPath [
            xfsprogs
            thin-provisioning-tools
            udev
            clevis
            jose
            jq
            cryptsetup
            curl
            tpm2-tools
            coreutils
          ];
        };
      });
}
