#!/bin/bash 
cargo test dirs -- --nocapture --test-threads 1
cargo test tests::get_path_core -- --nocapture 
cargo test cli -- --nocapture 
