#!/usr/bin/env bash

set -e

# This script performs computations and plots the results

echo -e "\n${PAD}Performing...\n"

echo -e "${PAD}Step 1. Instantiate the project"
echo -e "\n${PAD}> Building the Rust binary..."
cargo build -r &>/dev/null
echo -e "${PAD}> Instantiating the Julia project..."
julia --project=. -e "using Pkg; Pkg.instantiate()" &>/dev/null

echo -e "\n${PAD}Step 2. Perform computations"

mkdir -p "${D_ALL}" && cp "${I_ALL}" "${D_ALL}/input.dat"
mkdir -p "${D_SOLAR}" && cp "${I_SOLAR}" "${D_SOLAR}/input.dat"
mkdir -p "${D_HMSFRS}" && cp "${I_HMSFRS}" "${D_HMSFRS}/input.dat"
mkdir -p "${D_HMSFRS_TEST}" && cp "${I_HMSFRS_TEST}" "${D_HMSFRS_TEST}/input.dat"
mkdir -p "${D_VERA_REID}" && cp "${I_VERA_REID}" "${D_VERA_REID}/input.dat"

echo -e "\n${PAD}> Compute per-object data..."
"${PMG}" -i "${I_ALL}" -o "${DO_ALL}" --goal objects
"${PMG}" -i "${I_SOLAR}" -o "${DO_SOLAR}" --goal objects
"${PMG}" -i "${I_HMSFRS}" -o "${DO_HMSFRS}" --goal objects
"${PMG}" -i "${I_HMSFRS_TEST}" -o "${DO_HMSFRS_TEST}" --goal objects \
  --u-sun 11 --theta-sun 255 --r-0 8.34

echo -e "${PAD}> Fit all..."
"${PMG}" -i "${I_ALL}" -o "${DO_ALL}" --goal fit
echo -e "${PAD}> Fit near the solar circle (with errors)..."
"${PMG}" -i "${I_SOLAR}" -o "${DO_SOLAR}" --goal fit
echo -e "${PAD}> Fit HMSFRs..."
"${PMG}" -i "${I_HMSFRS}" -o "${DO_HMSFRS}" --goal fit

echo -e "
${PAD}Step 3. Plot the comparison charts for the objects that are
${PAD}        identified as the same in the first VERA catalogue (2020)
${PAD}        and the catalogue from Reid et al. (2019)"

"${JULIA}" "${COMPARE}" -i "${I_VERA_REID}" -o "${P_VERA_REID@Q}"

echo -e "${PAD}Step 4. Plot projections in each plane"

echo -e "\n${PAD}All by type:"
"${JULIA}" "${PROJECTIONS}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_TYPE@Q}"
echo -e "${PAD}All by source:"
"${JULIA}" "${PROJECTIONS}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_SOURCE@Q}" -s
echo -e "${PAD}Near the solar circle:"
"${JULIA}" "${PROJECTIONS}" -i "${DO_SOLAR@Q}" -o "${P_SOLAR@Q}" -s
echo -e "${PAD}HMSFRs:"
"${JULIA}" "${PROJECTIONS}" -i "${DO_HMSFRS@Q}" -o "${P_HMSFRS@Q}" -s

echo "${PAD}Step 5. Plot the rotation curves"

echo -e "\n${PAD}All by type:"
"${JULIA}" "${ROTCURVE}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_TYPE@Q}"
"${JULIA}" "${FIT_ROTCURVE}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_TYPE@Q}"
echo -e "${PAD}All by source:"
"${JULIA}" "${ROTCURVE}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_SOURCE@Q}" -s
"${JULIA}" "${FIT_ROTCURVE}" -i "${DO_ALL@Q}" -o "${P_ALL_BY_SOURCE@Q}" -s
echo -e "${PAD}Near the solar circle:"
"${JULIA}" "${ROTCURVE}" -i "${DO_SOLAR@Q}" -o "${P_SOLAR@Q}" -s
"${JULIA}" "${FIT_ROTCURVE}" -i "${DO_SOLAR@Q}" -o "${P_SOLAR@Q}" -s
echo -e "${PAD}HMSFRs:"
"${JULIA}" "${ROTCURVE}" -i "${DO_HMSFRS@Q}" -o "${P_HMSFRS@Q}" -s
"${JULIA}" "${FIT_ROTCURVE}" -i "${DO_HMSFRS@Q}" -o "${P_HMSFRS@Q}" -s
echo -e "${PAD}HMSFRs (test):"
"${JULIA}" "${ROTCURVE}" -i "${DO_HMSFRS_TEST@Q}" -o "${P_HMSFRS_TEST@Q}" -s --with-test

echo -e "${PAD}Step 6. Zip the results\n"

zip -rq results.zip results
