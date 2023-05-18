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

mkdir -p "${R_ALL}" && cp "${I_ALL}" "${R_ALL}/input.dat"
mkdir -p "${R_SOLAR}" && cp "${I_SOLAR}" "${R_SOLAR}/input.dat"
mkdir -p "${R_HMSFRS}" && cp "${I_HMSFRS}" "${R_HMSFRS}/input.dat"
mkdir -p "${R_HMSFRS_TEST}" && cp "${I_HMSFRS_TEST}" "${R_HMSFRS_TEST}/input.dat"
mkdir -p "${R_VERA_REID}" && cp "${I_VERA_REID}" "${R_VERA_REID}/input.dat"

echo -e "\n${PAD}> Compute per-object data..."
"${PMG}" -i "${I_ALL}" -o "${R_ALL}" --goal objects
"${PMG}" -i "${I_SOLAR}" -o "${R_SOLAR}" --goal objects
"${PMG}" -i "${I_HMSFRS}" -o "${R_HMSFRS}" --goal objects
"${PMG}" -i "${I_HMSFRS_TEST}" -o "${R_HMSFRS_TEST}" --goal objects \
  --u-sun 11 --theta-sun 255 --r-0 8.34

echo -e "${PAD}> Fit near the solar circle (disabled inner optimization, with errors and profiles)..."
"${PMG}" -i "${I_SOLAR}" -o "${R_SOLAR_DI}" --goal fit \
  --n-best "${N_BEST_SOLAR}" \
  --n-max "${N_MAX_SOLAR}" \
  --with-errors \
  --with-conditional-profiles \
  --disable-inner 2>/dev/null
echo -e "${PAD}> Fit HMSFRs (disabled inner optimization, with errors and profiles)..."
"${PMG}" -i "${I_HMSFRS}" -o "${R_HMSFRS_DI}" --goal fit \
  --n-best "${N_BEST_HMSFRS}" \
  --n-max "${N_MAX_HMSFRS}" \
  --with-errors \
  --with-conditional-profiles \
  --disable-inner \
  --lbfgs-tolerance 1e-12 2>/dev/null
echo -e "${PAD}> Fit near the solar circle (with errors and profiles)..."
"${PMG}" -i "${I_SOLAR}" -o "${R_SOLAR}" --goal fit \
  --n-best "${N_BEST_SOLAR}" \
  --n-max "${N_MAX_SOLAR}" \
  --with-errors \
  --with-conditional-profiles 2>/dev/null
echo -e "${PAD}> Fit HMSFRs (with errors and profiles)..."
"${PMG}" -i "${I_HMSFRS}" -o "${R_HMSFRS}" --goal fit \
  --n-best "${N_BEST_HMSFRS}" \
  --n-max "${N_MAX_HMSFRS}" \
  --with-errors \
  --with-conditional-profiles 2>/dev/null

echo -e "
${PAD}Step 3. Plot the comparison charts for the objects that are
${PAD}        identified as the same in the first VERA catalogue (2020)
${PAD}        and the catalogue from Reid et al. (2019)"

"${JULIA}" "${COMPARE}" -i "${I_VERA_REID}" -o "${R_VERA_REID@Q}"

echo -e "${PAD}Step 4. Plot projections in each plane"

echo -e "\n${PAD}All by type:"
"${JULIA}" "${PROJECTIONS}" -i "'${R_ALL}/objects'" -o "'${R_ALL}/objects/Plots by type'"
echo -e "${PAD}All by source:"
"${JULIA}" "${PROJECTIONS}" -i "'${R_ALL}/objects'" -o "'${R_ALL}/objects/Plots by source'" -s
echo -e "${PAD}Near the solar circle:"
"${JULIA}" "${PROJECTIONS}" -i "'${R_SOLAR}/objects'" -o "'${R_SOLAR}/objects'" -s
echo -e "${PAD}HMSFRs:"
"${JULIA}" "${PROJECTIONS}" -i "'${R_HMSFRS}/objects'" -o "'${R_HMSFRS}/objects'" -s

echo "${PAD}Step 5. Plot the rotation curves"

echo -e "\n${PAD}All by type:"

"${JULIA}" "${ROTCURVE}" -i "'${R_ALL}/objects'" -o "'${R_ALL}/objects/Plots by type'"

echo -e "${PAD}All by source:"

"${JULIA}" "${ROTCURVE}" -i "'${R_ALL}/objects'" -o "'${R_ALL}/objects/Plots by source'" -s

echo -e "${PAD}Near the solar circle (disabled inner optimization):\n"

for i in $(seq 1 "${N_MAX_SOLAR}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${FIT_ROTCURVE}" -i "'${R_SOLAR_DI}'" -o "'${R_SOLAR_DI}'" -s -n "$i"
done
qpdf --empty --pages "${R_SOLAR_DI}"/*/"Fitted rotation curve.pdf" -- "${R_SOLAR_DI}/Fitted rotation curves.pdf"
qpdf --empty --pages "${R_SOLAR_DI}"/*/"Fitted rotation curve (errors).pdf" -- "${R_SOLAR_DI}/Fitted rotation curves (errors).pdf"

echo -e "${PAD}HMSFRs (disabled inner optimization):\n"

for i in $(seq 1 "${N_MAX_HMSFRS}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${FIT_ROTCURVE}" -i "'${R_HMSFRS_DI}'" -o "'${R_HMSFRS_DI}'" -s -n "$i"
done
qpdf --empty --pages "${R_HMSFRS_DI}"/*/"Fitted rotation curve.pdf" -- "${R_HMSFRS_DI}/Fitted rotation curves.pdf"
qpdf --empty --pages "${R_HMSFRS_DI}"/*/"Fitted rotation curve (errors).pdf" -- "${R_HMSFRS_DI}/Fitted rotation curves (errors).pdf"

echo -e "${PAD}Near the solar circle:"

"${JULIA}" "${ROTCURVE}" -i "'${R_SOLAR}/objects'" -o "'${R_SOLAR}/objects'" -s
for i in $(seq 1 "${N_MAX_SOLAR}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${FIT_ROTCURVE}" -i "'${R_SOLAR}'" -o "'${R_SOLAR}'" -s -n "$i"
done
qpdf --empty --pages "${R_SOLAR}"/*/"Fitted rotation curve.pdf" -- "${R_SOLAR}/Fitted rotation curves.pdf"
qpdf --empty --pages "${R_SOLAR}"/*/"Fitted rotation curve (errors).pdf" -- "${R_SOLAR}/Fitted rotation curves (errors).pdf"

echo -e "${PAD}HMSFRs:"

"${JULIA}" "${ROTCURVE}" -i "'${R_HMSFRS}/objects'" -o "'${R_HMSFRS}/objects'" -s
for i in $(seq 1 "${N_MAX_HMSFRS}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${FIT_ROTCURVE}" -i "'${R_HMSFRS}'" -o "'${R_HMSFRS}'" -s -n "$i"
done
qpdf --empty --pages "${R_HMSFRS}"/*/"Fitted rotation curve.pdf" -- "${R_HMSFRS}/Fitted rotation curves.pdf"
qpdf --empty --pages "${R_HMSFRS}"/*/"Fitted rotation curve (errors).pdf" -- "${R_HMSFRS}/Fitted rotation curves (errors).pdf"

echo -e "${PAD}HMSFRs (test):"
"${JULIA}" "${ROTCURVE}" -i "'${R_HMSFRS_TEST}/objects'" -o "'${R_HMSFRS_TEST}/objects'" -s --with-test

echo "${PAD}Step 6. Plot the profiles"

echo -e "\n${PAD}Near the solar circle (disabled inner optimization):\n"

for i in $(seq 1 "${N_MAX_SOLAR}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${PROFILES}" -i "'${R_SOLAR_DI}'" -o "'${R_SOLAR_DI}'" -n "$i"
done

echo -e "${PAD}HMSFRs (disabled inner optimization):\n"

for i in $(seq 1 "${N_MAX_HMSFRS}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${PROFILES}" -i "'${R_HMSFRS_DI}'" -o "'${R_HMSFRS_DI}'" -n "$i"
done

echo -e "${PAD}Near the solar circle:\n"

for i in $(seq 1 "${N_MAX_SOLAR}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${PROFILES}" -i "'${R_SOLAR}'" -o "'${R_SOLAR}'" -n "$i"
done

echo -e "${PAD}HMSFRs:\n"

for i in $(seq 1 "${N_MAX_HMSFRS}"); do
  echo -e "${PAD}n = $i"
  "${JULIA}" "${PROFILES}" -i "'${R_HMSFRS}'" -o "'${R_HMSFRS}'" -n "$i"
done

echo -e "${PAD}Step 7. Plot the \`n\` plots"

"${JULIA}" "${N}" -i "'${R_SOLAR_DI}'" -o "'${R_SOLAR_DI}'"
"${JULIA}" "${N}" -i "'${R_HMSFRS_DI}'" -o "'${R_HMSFRS_DI}'"
"${JULIA}" "${N}" -i "'${R_SOLAR}'" -o "'${R_SOLAR}'"
"${JULIA}" "${N}" -i "'${R_HMSFRS}'" -o "'${R_HMSFRS}'"

echo -e "${PAD}Step 8. Plot the inner profiles and odd objects"

"${JULIA}" "${INNER_PROFILES}" -i "'${R_SOLAR}'" -o "'${R_SOLAR}'" -n "${N_BEST_SOLAR}"
"${JULIA}" "${INNER_PROFILES}" -i "'${R_HMSFRS}'" -o "'${R_HMSFRS}'" -n "${N_BEST_HMSFRS}"

"${JULIA}" "${ODD_OBJECTS}" -i "'${R_SOLAR}'" -o "'${R_SOLAR}'" -n "${N_BEST_SOLAR}"
"${JULIA}" "${ODD_OBJECTS}" -i "'${R_HMSFRS}'" -o "'${R_HMSFRS}'" -n "${N_BEST_HMSFRS}"

echo -e "${PAD}Step 9. Zip the results\n"

rm -f results.zip
zip -rq results.zip results
