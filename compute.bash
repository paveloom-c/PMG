#!/usr/bin/env bash

set -e

# This script performs computations and plots the results

PAD="    "

echo -e "\n${PAD}Performing...\n"

echo "${PAD}Step 1. Instantiate the project"
cargo build -r &>/dev/null
julia --project=. -e "using Pkg; Pkg.instantiate()"

echo -e "\n${PAD}Step 2. Perform computations"

echo -e "\n${PAD}> Compute per-object data..."
cargo run -r -- -o data/output/all --goal objects -i data/input/all.dat &>/dev/null
cargo run -r -- -o data/output/near_the_solar_circle --goal objects -i data/input/near_the_solar_circle.dat &>/dev/null
cargo run -r -- -o data/output/hmsfrs --goal objects -i data/input/hmsfrs.dat &>/dev/null
cargo run -r -- -o data/output/hmsfrs_test --goal objects -i data/input/hmsfrs.dat \
  --u-sun 11 --theta-sun 255 --r-0 8.34 &>/dev/null

echo -e "${PAD}> Fit all..."
cargo run -r -- -o data/output/all --goal fit -i data/input/all.dat &>/dev/null
echo -e "${PAD}> Fit near the solar circle (with errors)..."
cargo run -r -- -o data/output/near_the_solar_circle --goal fit-with-errors -i data/input/near_the_solar_circle.dat &>/dev/null
echo -e "${PAD}> Fit HMSFRs..."
cargo run -r -- -o data/output/hmsfrs --goal fit -i data/input/hmsfrs.dat &>/dev/null

echo -e "
${PAD}Step 3. Plot the comparison charts for the objects that are
${PAD}        identified as the same in the first VERA catalogue (2020)
${PAD}        and the catalogue from Reid et al. (2019)"

./julia.bash scripts/compare.jl -o "'VERA vs. Reid'"

echo -e "${PAD}Step 4. Plot projections in each plane"

echo -e "\n${PAD}All by type:"
./julia.bash scripts/projections.jl -o "'All by type'" data/output/all
echo -e "${PAD}All by source:"
./julia.bash scripts/projections.jl -s -o "'All by source'" data/output/all
echo -e "${PAD}Near the solar circle:"
./julia.bash scripts/projections.jl -s -o "'Near the solar circle'" data/output/near_the_solar_circle
echo -e "${PAD}HMSFRs:"
./julia.bash scripts/projections.jl -s -o "HMSFRs" data/output/hmsfrs

echo "${PAD}Step 5. Plot the rotation curves"

echo -e "\n${PAD}All by type:"
./julia.bash scripts/rotcurve.jl -o "'All by type'" data/output/all
./julia.bash scripts/fit_rotcurve.jl -o "'All by type'" data/output/all
echo -e "${PAD}All by source:"
./julia.bash scripts/rotcurve.jl -s -o "'All by source'" data/output/all
./julia.bash scripts/fit_rotcurve.jl -s -o "'All by source'" data/output/all
echo -e "${PAD}Near the solar circle:"
./julia.bash scripts/rotcurve.jl -s -o "'Near the solar circle'" data/output/near_the_solar_circle
./julia.bash scripts/fit_rotcurve.jl -s -o "'Near the solar circle'" data/output/near_the_solar_circle
echo -e "${PAD}HMSFRs:"
./julia.bash scripts/rotcurve.jl -s -o "HMSFRs" data/output/hmsfrs/
./julia.bash scripts/fit_rotcurve.jl -s -o "HMSFRs" data/output/hmsfrs/
echo -e "${PAD}HMSFRs (test):"
./julia.bash scripts/rotcurve.jl -s --with-test -o "'HMSFRs (Test)'" data/output/hmsfrs_test

echo -e "${PAD}Step 6. Zip the results\n"

zip -rq results.zip data plots
