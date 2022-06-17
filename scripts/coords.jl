# This script converts the new data to the
# Galactic heliocentric Cartesian system and plots
# the projections in each plane

# Prepare color codes
RESET = "\e[0m"
GREEN = "\e[32m"
YELLOW = "\e[33m"

# Check for required arguments
if length(ARGS) != 1
    println("""
        $(YELLOW)USAGE:$(RESET)
            { julia --project=. | ./julia.bash } scripts/coords.jl <OUTPUT>

        $(YELLOW)ARGS:$(RESET)
            $(GREEN)<OUTPUT>$(RESET)    Output directory with data files
                        (relative to the root of the repository)"""
    )
    exit(1)
end

# Define the output directory
OUTPUT_DIR = ARGS[end]

"Padding in the output"
pad = 4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = UInt64

println('\n', " "^pad, "> Loading the packages...")

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
COORDS_DATA_PATH = joinpath(ROOT_DIR, OUTPUT_DIR, "bin", "coords.bin")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the data files
println(" "^pad, "> Loading the data...")

struct Data
    names::Vector{String}
    x::Vector{F}
    y::Vector{F}
    z::Vector{F}
    obj_type::Vector{String}
end

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Data
    open(path, "r") do io
        # Read the number of objects
        n = read(io, I)
        # Get the fields and their types
        fields = fieldnames(Data)
        types = eltype.(fieldtypes(Data))
        # Initialize the data struct
        data = Data(ntuple(_ -> [], length(fields))...)
        # For each object
        for _ in 1:n
            # For each field
            for (field, type) in zip(fields, types)
                # If the type is a string
                v = if type == String
                    # Read the number of bytes
                    nbytes = read(io, I)
                    # Read the string
                    String(read(io, nbytes))
                # Otherwise,
                else
                    # Read the value
                    read(io, type)
                end
                # Save the value
                push!(getfield(data, field), v)
            end
        end
        data
    end
end

# Read the data
data = read_bincode(COORDS_DATA_PATH)

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
