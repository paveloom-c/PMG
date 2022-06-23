#!/bin/bash

# Exit when any command fails
set -e

# This script performs computations and plots the results

PAD="    "

echo -e "\n${PAD}Performing...\n"

echo "${PAD}Step 1. Instantiate the project"
cargo build -r &>/dev/null
julia --project=. -e "using Pkg; Pkg.instantiate()"


echo "
${PAD}Step 2. Convert the equatorial spherical coordinates
${PAD}        to the Galactic heliocentric spherical and
${PAD}        Cartesian coordinates, compute the distances"

cargo run -r -- -o data/output/all --goals coords -i data/input/all.dat &>/dev/null
cargo run -r -- -o data/output/hmsfrs --goals coords -i data/input/hmsfrs.dat &>/dev/null

echo "
${PAD}Step 3. Plot projections in each plane"

echo -e "\n${PAD}All by type:"
./julia.bash scripts/coords.jl -o "'All by type'" data/output/all/
echo -e "${PAD}All by source:"
./julia.bash scripts/coords.jl -s -o "'All by source'" data/output/all/
echo -e "${PAD}HMSFRs:"
./julia.bash scripts/coords.jl -s -o "HMSFRs" data/output/hmsfrs/
