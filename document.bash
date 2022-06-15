#!/bin/bash

# A script to document the crate

# Document the crate
cargo doc --workspace --document-private-items
RUSTDOCFLAGS="--html-in-header assets/katex-header.html" cargo doc --no-deps --workspace --document-private-items

# Copy the documentation to the output directory
mv target/doc public

# Redirect to the first crate's reference
echo "<meta http-equiv=\"refresh\" content=\"0; url=pmg\">" >public/index.html
