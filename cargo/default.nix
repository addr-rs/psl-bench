let

  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  latest = nixpkgs.rustChannelOf {
    channel = "stable";
    date = "2018-07-10";
  };

in

with nixpkgs;

stdenv.mkDerivation {
  name = "psl";
  buildInputs = [ latest.rust pypy ];
  propagatedBuildInputs = [ python36Packages.publicsuffix ];
  OPENSSL_DIR = "${openssl.dev}";
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
}
