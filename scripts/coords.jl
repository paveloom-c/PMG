# This script converts the new data to the
# Galactic heliocentric Cartesian system and plots
# the projections in each plane

"Padding in the output"
pad = 4

"Floating point type used across the script"
F = Float64

println('\n', " "^pad, "> Loading the packages...")

using CSV
using LaTeXStrings
using Plots

# Use the PGFPlotsX backend for plots
pgfplotsx()

# Change some of the default parameters for plots
default(
    fontfamily="Computer Modern",
    dpi=300,
    size=(300, 300),
    markersize=2.5,
    legend=:topright,
)

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
PLOTS_DIR = joinpath(ROOT_DIR, "plots")
OUTPUT_DATA_DIR = joinpath(ROOT_DIR, "data", "output")
COORDS_DATA_PATH = joinpath(OUTPUT_DATA_DIR, "coords.dat")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the data files
println(" "^pad, "> Loading the data...")

# Read the data
data = CSV.File(
    COORDS_DATA_PATH,
    delim=' ',
    types=[String, F, F, F, String],
)

println(" "^pad, "> Plotting the scatter plots...")

# Plot a scatter plot in the (X, Y) plane
scatter(
    data.x,
    data.y,
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Y \; \mathrm{[kpc]}",
    group=data.obj_type,
)
savefig(joinpath(PLOTS_DIR, "XY.pdf"))

# Plot a scatter plot in the (X, Z) plane
scatter(
    data.x,
    data.z,
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    group=data.obj_type,
)
savefig(joinpath(PLOTS_DIR, "XZ.pdf"))

# Plot a scatter plot in the (Y, Z) plane
scatter(
    data.y,
    data.z,
    xlabel=L"Y \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    group=data.obj_type,
    legend=:topleft,
)
savefig(joinpath(PLOTS_DIR, "YZ.pdf"))

println()
