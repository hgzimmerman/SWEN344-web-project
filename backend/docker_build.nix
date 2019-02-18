with import <nixpkgs> {};


pkgs.dockerTools.buildImage {
    name = "swen344";
    tag = "latest";
    contents = [
        pkgs.htop
        pkgs.curl
        pkgs.cacert
        pkgs.bashInteractive
        pkgs.rustup
        pkgs.openssl
        pkgs.postgresql
        pkgs.pkgconfig
        pkgs.coreutils
    ];
    config = {
        Env = [
            "NIX_PAGER=cat"
            "DATABASE_URL=postgres://hzimmerman:password@localhost/web_engineering"
            "TEST_DATABASE_URL=postgres://hzimmerman:password@localhost/web_engineering_test"
            "TEST_DATABASE_NAME=web_engineering_test"
            "DROP_DATABASE_URL=postgres://hzimmerman:password@localhost/postgres"
        ];
        Cmd = [ "/bin/bash" ];
    };
    runAsRoot = ''
        echo "(runAsRoot)" > runAsRoot.yeet
        rustup default nightly
        rustup update
    '';
}

