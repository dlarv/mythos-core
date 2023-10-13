#!/bin/bash 
cargo test dirs -- --nocapture --test-threads 1
cargo test lib -- --nocapture --test-threads 1
