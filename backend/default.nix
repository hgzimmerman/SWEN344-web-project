with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "template";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    postgresql

    openssl 
    # these two are optional, but they help with installing some rust Programs 
    pkgconfig
  ];
  shellHook = ''
    export PGDATA='pgsql'
    # to set the password, run `psql` and enter `\password` and set it to the password below
    export DATABASE_URL='postgres://hzimmerman:password@localhost/web_engineering'
    export TEST_DATABASE_URL='postgres://hzimmerman:password@localhost/web_engineering_test'
    export TEST_DATABASE_NAME='web_engineering_test'
    export DROP_DATABASE_URL='postgres://hzimmerman:password@localhost/postgres'

    pg_ctl init
    pg_ctl -l db.logfile start -o "-h localhost -i"

    alias docs='cargo rustdoc --bins --open -- --document-private-items'
  '';
}
