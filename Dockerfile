FROM lnl7/nix:2.1.2

RUN nix-env -iA \
 nixpkgs.curl \
 nixpkgs.rustup \
 nixpkgs.bashInteractive

ADD . /Swen344
