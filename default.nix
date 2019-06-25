let
  defaultPkgs = import <nixpkgs> {};
in

{
  openssl ? defaultPkgs.openssl,
  pkg-config ? defaultPkgs.pkg-config,
  rustPlatform ? defaultPkgs.rustPlatform
}:

rustPlatform.buildRustPackage rec {
  name = "noorton-${version}";
  version = "unstable";

  src = ./.;

  cargoSha256 = "0n8cbf4ifsiv0dz0y9jkvd5ihinhlj6911nfjj7a6kd5c3phwi1b";

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
}
