{ pkgs }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
    clippy
    rustfmt
    
    pkg-config
    openssl
    
    cargo-watch
    cargo-edit
    cargo-audit
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    systemd
    udev
    procps
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.IOKit
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustc.src}/lib/rustlib/src/rust/library";
}