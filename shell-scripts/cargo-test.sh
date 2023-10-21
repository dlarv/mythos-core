#!/bin/bash 
cargo test tests::get_path_core -- --nocapture --test-threads 1
cargo test dirs -- --nocapture --test-threads 1
cargo test conf -- --nocapture --test-threads 1
cargo test cli -- --nocapture 
