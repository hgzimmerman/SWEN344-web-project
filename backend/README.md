## SWEN 344 Web Engineering Project Group 3


### Run Backend Locally
I will try to make this as platform agnostic as possible,
but the backend was developed on Linux (Nixos),
so some details may require some tweaking.

Make sure you have the openssl library installed as well as postgres and its associated libraries.
Install nightly rust.
Do this by running `curl https://sh.rustup.rs -sSf | sh` and following its directions to install the nightly release of Rust.

Get postgres running by first setting these environment variables, where `<USERNAME>` is replaced with your username:
```
export PGDATA='pgsql'
export DATABASE_URL='postgres://<USERNAME>:password@localhost/web_engineering'
export TEST_DATABASE_URL='postgres://<USERNAME>:password@localhost/web_engineering_test'
export DROP_DATABASE_URL='postgres://<USERNAME>:password@localhost/postgres'
```

Then navigate to the `/backend/` directory and run:
```
pg_ctl init
pg_ctl -l db.logfile start -o "-h localhost -i"
```

Install the diesel cli tool by running `cargo install diesel_cli --no-default-features --features "postgres" -f`.
Navigate to the `/backend/server/` directory of the project and run diesel migration run.

Navigate to the `/backend/server/` directory of the project and run `cargo run --release`.