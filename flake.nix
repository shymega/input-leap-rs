# SPDX-FileCopyrightText: 2023 The Input-Leap Developers
#
# SPDX-License-Identifier: GPL-2.0-only

{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config cmake ] ;
          buildInputs = with pkgs; [ systemd.dev dbus.dev ];
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo pkg-config cmake ];
          buildInputs = with pkgs; [ systemd.dev dbus.dev ];
        };
      }
    );
}
