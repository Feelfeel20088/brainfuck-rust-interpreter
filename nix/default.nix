{ 
  lib
, rustPlatform
, pkg-config
}:

rustPlatform.buildRustPackage rec {
  pname = "brainfuck-rust-interpreter";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];

  meta = with lib; {
    description = "A tiny Brainfuck interpreter written in Rust";
    homepage = null;
    license = licenses.mit;
    maintainers = [];
    platforms = platforms.unix;
    mainProgram = "brainfuck-rust-interpreter";
  };
}

