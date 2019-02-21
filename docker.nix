with import <nixpkgs> {};


pkgs.dockerTools.buildImage {
    name = "swen344";
    tag = "latest";
    contents = [
        pkgs.nix
        pkgs.htop
        pkgs.curl
        pkgs.cacert # Needed for SSL connections to work for setting up rust
        pkgs.bashInteractive
        pkgs.rustup
        pkgs.openssl
        pkgs.postgresql
        pkgs.pkgconfig
        pkgs.coreutils
    ];

    config = {
        Env = [
            "DATABASE_URL=postgres://hzimmerman:password@localhost/web_engineering"
            "TEST_DATABASE_URL=postgres://hzimmerman:password@localhost/web_engineering_test"
            "TEST_DATABASE_NAME=web_engineering_test"
            "DROP_DATABASE_URL=postgres://hzimmerman:password@localhost/postgres"
        ];
        Cmd = [ "/bin/bash" ];
    };
    runAsRoot = ''
        #!${stdenv.shell}
        echo "(runAsRoot)" > runAsRoot.yeet
        ${dockerTools.shadowSetup}
        groupadd -r nixbld
        mkdir /home
        mkdir /home/server
        useradd -m server --password password_yeet_420
    '';
}