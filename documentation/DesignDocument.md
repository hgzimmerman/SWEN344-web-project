### UML Class Diagram
### Sequence diagrams
##### Authentication
##### Stock Trading
##### Stock Trading Display

### Component Architecture
### Deployment Plan
1. Provision VM.
2. Clone/Copy the repository to the VM.
3. Install `rustup` to install cargo and rust.
4. Install `nix` package manager.
5. Install `npm`.
6. Run `nix-shell` to set up the db.
7. Install `cargo install diesel` to install the db management utility.
8. Build the frontend.
9. Build and run the backend with `cargo run --release` in the `/backend/server` directory.
### Test Plan
* Mostly integration tests on the backend.
* Likely no CI service, devs are expected to test locally before merging to master.
* Unit and webdriver tests on the frontend.