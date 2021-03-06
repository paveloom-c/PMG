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
${PAD}Step 2. Perform computations:
${PAD}        - convert the equatorial spherical coordinates
${PAD}          to the Galactic heliocentric spherical and
${PAD}          Cartesian coordinates, compute the distances;
${PAD}        - compute the rotation curve"

cargo run -r -- -o data/output/all --goals coords rotcurve -i data/input/all.dat &>/dev/null
cargo run -r -- -o data/output/hmsfrs --goals coords rotcurve -i data/input/hmsfrs.dat &>/dev/null

echo -e "
${PAD}Step 3. Plot the comparison charts for the objects that are
${PAD}        identified as the same in the first VERA catalogue (2020)
${PAD}        and the catalogue from Reid et al. (2019)"

./julia.bash scripts/compare.jl -o "'VERA vs. Reid'"

echo -e "${PAD}Step 4. Plot projections in each plane"

echo -e "\n${PAD}All by type:"
./julia.bash scripts/coords.jl -o "'All by type'" data/output/all/
echo -e "${PAD}All by source:"
./julia.bash scripts/coords.jl -s -o "'All by source'" data/output/all/
echo -e "${PAD}HMSFRs:"
./julia.bash scripts/coords.jl -s -o "HMSFRs" data/output/hmsfrs/

echo "${PAD}Step 5. Plot the rotation curve"

echo -e "\n${PAD}All by type:"
./julia.bash scripts/rotcurve.jl -o "'All by type'" data/output/all/
echo -e "${PAD}All by source:"
./julia.bash scripts/rotcurve.jl -s -o "'All by source'" data/output/all/
echo -e "${PAD}HMSFRs:"
./julia.bash scripts/rotcurve.jl -s -o "HMSFRs" data/output/hmsfrs/
