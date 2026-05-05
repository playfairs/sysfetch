{ pkgs }:

pkgs.rustPlatform.buildRustPackage {
  pname = "sysfetch";
  version = "0.1.0";
  
  src = ./.;
  
  cargoLock.lockFile = ./Cargo.lock;
  
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustc
    cargo
  ];
  
  buildInputs = with pkgs; [
    openssl
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    systemd
    udev
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.IOKit
    darwin.apple_sdk.frameworks.SystemConfiguration
    libiconv
  ];
  
  meta = with pkgs.lib; {
    description = "A Minimal Fetching Utility.";
    homepage = "https://github.com/playfairs/sysfetch";
    license = licenses.gpl3;
    platforms = platforms.unix;
    mainProgram = "sysfetch";
  };
}