# This script converts the new data to the
# Galactic heliocentric Cartesian system and plots
# the projections in each plane

"Padding in the output"
pad = 4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = Int

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
)

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
PLOTS_DIR = joinpath(ROOT_DIR, "plots")
INPUT_DATA_DIR = joinpath(ROOT_DIR, "data", "input")
OUTPUT_DATA_DIR = joinpath(ROOT_DIR, "data", "output")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)
mkpath(OUTPUT_DATA_DIR)

"Convert an hours-minutes-seconds angle to radians"
function hms2rad(hms::Tuple{F, F, F})::F where F <: Real
    return deg2rad(hms[1] * 15 + hms[2] / 4 + hms[3] / 240)
end

"Convert a degrees-minutes-seconds angle to radians"
function dms2rad(deg::Tuple{F, F, F})::F where F <: Real
    return deg2rad((abs(deg[1]) + deg[2] / 60 + deg[3] / 3600) * sign(deg[1]))
end

"Convert a string that holds an angle to a tuple"
function str2tuple(str::S)::Tuple{F, F, F} where S <: AbstractString
    return tuple([parse(F, p) for p in split(str)]...)
end

"The right ascension of the north galactic pole"
α_NGP = hms2rad((12., 51., 26.2817))

"The declination of the north galactic pole"
δ_NGP = dms2rad((27., 7., 42.013))

"The longitude of the north celestial pole"
l_NCP = 122.932

"""
Convert the coordinates in the equatorial spherical system (right ascension
and declination) to the coordinates in the Galactic heliocentric Cartesian
system (longitude and latitude) using the value of parallax
"""
function convert(α::Tuple{F, F, F}, δ::Tuple{F, F, F}, par::F) where F <: Real
    # Convert the angles to radians
    α = hms2rad(α)
    δ = dms2rad(δ)
    # Convert to the Galactic heliocentric spherical system
    φ = atan(
        cos(δ) * sin(α - α_NGP),
        cos(δ_NGP) * sin(δ) - sin(δ_NGP) * cos(δ) * cos(α - α_NGP),
    )
    b = asin(sin(δ_NGP) * sin(δ) + cos(δ_NGP) * cos(δ) * cos(α - α_NGP))
    l = l_NCP - φ
    # Compute the distance in kpc
    d = 1 / par
    # Convert to the Galactic heliocentric Cartesian system
    x = d * cos(b) * cos(l)
    y = d * cos(b) * sin(l)
    z = d * sin(b)
    return x, y, z
end

# Prepare storage for the data
o = I[]
names = String[]
X = F[]
Y = F[]
Z = F[]

# Define the names of the data sets
data_names = ["HMSFR", "Non-HMSFR"]

# Define the paths to the data files
data_files = joinpath.(INPUT_DATA_DIR, lowercase.(data_names) .* ".dat")

println(" "^pad, "> Loading the data...")

# For each data file
for data_file in data_files
    # Read the data
    file = CSV.File(
        data_file,
        delim=" "^4,
        skipto=2,
        stringtype=String,
        header=[
            "name",
            "α",
            "δ",
            "par",
            "σ_par",
            "V_r",
            "σ_V_r",
            "μ_x",
            "σ_μ_x",
            "μ_y",
            "σ_μ_y",
            "ref",
        ],
    )
    # Parse the right ascensions and declinations
    α = map(str2tuple, file.α)
    δ = map(str2tuple, file.δ)
    # Parse the parallaxes
    par = map(x -> parse(Float64, x), file.par)
    # For each object
    for (α, δ, par) in zip(α, δ, par)
        # Convert to the Galactic heliocentric Cartesian system
        x, y, z = convert(α, δ, par)
        # Push the results
        push!(X, x)
        push!(Y, y)
        push!(Z, z)
    end
    # Mark the offset
    push!(o, length(X))
    # Save the names
    push!(names, file.name...)
end

println(" "^pad, "> Writing the results...")

# Write the results
open(joinpath(OUTPUT_DATA_DIR, "xyz.dat"), "w") do io
    delim = " "^4
    println(
        io,
        "Name",
        " "^(length(names[1]) - 4) * delim,
        "X",
        " "^(length("$(X[1])") - 1) * delim,
        "Y",
        " "^(length("$(Y[1])") - 1) * delim,
        "Z",
    )
    for (name, x, y, z) in zip(names, X, Y, Z)
        println(io, name, delim, x, delim, y, delim, z)
    end
end

println(" "^pad, "> Plotting the scatter plots...")

# Plot a scatter plot in the (X, Y) plane
scatter(
    X[1:o[1]],
    Y[1:o[1]],
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Y \; \mathrm{[kpc]}",
    label=data_names[1],
    legend=:topleft,
)
scatter!(
    X[o[1]+1:end],
    Y[o[1]+1:end],
    label=data_names[2],
)
savefig(joinpath(PLOTS_DIR, "XY.pdf"))

# Plot a scatter plot in the (X, Z) plane
scatter(
    X[1:o[1]],
    Z[1:o[1]],
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    label=data_names[1],
)
scatter!(
    X[o[1]+1:end],
    Z[o[1]+1:end],
    label=data_names[2],
)
savefig(joinpath(PLOTS_DIR, "XZ.pdf"))

# Plot a scatter plot in the (Y, Z) plane
scatter(
    Y[1:o[1]],
    Z[1:o[1]],
    xlabel=L"Y \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    label=data_names[1],
)
scatter!(
    Y[o[1]+1:end],
    Z[o[1]+1:end],
    label=data_names[2],
)
savefig(joinpath(PLOTS_DIR, "YZ.pdf"))

println()
