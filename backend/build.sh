#!/usr/bin/env bash

cd ../frontend;
npm install && npm run-script build
cd ../backend
bash ./wait_for_it.sh db:5432 -q -- diesel setup --migration-dir db/migrations && cargo run --release -- --port 80