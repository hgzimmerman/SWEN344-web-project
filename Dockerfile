FROM lnl7/nix:2.1.2


RUN nix-env -iA \
 nixpkgs.linux-pam \
 nixpkgs.curl \
 nixpkgs.rustup \
 nixpkgs.bashInteractive \
 nixpkgs.nodejs

RUN useradd --create-home --password password server

USER server
WORKDIR /home/server


