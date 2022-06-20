# This script converts the new data to the
# Galactic heliocentric Cartesian system and plots
# the projections in each plane

"Check if the value of the option is the last argument"
function check_last(i)
    if i == length(ARGS)
        println("The last argument is reserved.")
        exit(1)
    end
end

"Parse the string, taking more arguments if it's quoted"
function parse_quoted_string(i)::String
    # Start from the first argument after the flag
    j = i
    # If the arguments starts with an apostrophe,
    s = if startswith(ARGS[i], "'")
        # Find the index of the argument
        # which ends with an apostrophe
        while !endswith(ARGS[j], "'")
            j += 1
        end
        # Join the arguments in one string
        # and remove the apostrophes
        chop(join(ARGS[i:j], ' '), head=1, tail=1)
    # Otherwise,
    else
        # Return the next argument
        ARGS[i]
    end
    # Check for the last argument
    check_last(j)
    return s
end

# Define default values for optional arguments
LEGEND_SHOW_SOURCES = false
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
    # Show sources on the legend instead of types
    if ARGS[i] == "-s"
        check_last(i)
        global LEGEND_SHOW_SOURCES = true
    end
    # A postfix for the names of output files
    if ARGS[i] == "--postfix"
        try
            global POSTFIX = " ($(parse_quoted_string(i+1)))"
        catch
            println("Couldn't parse the value of the `--postfix` argument.")
            exit(1)
        end
    end
end

# Prepare color codes
RESET = "\e[0m"
GREEN = "\e[32m"
YELLOW = "\e[33m"

# Check for required arguments
if length(ARGS) <= 1
    println("""
        $(YELLOW)USAGE:$(RESET)
            { julia --project=. | ./julia.bash } scripts/coords.jl [-s] <OUTPUT>

        $(YELLOW)ARGS:$(RESET)
            $(GREEN)<OUTPUT>$(RESET)    Output directory with data files
                        (relative to the root of the repository)

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-s$(RESET)    Show sources on the legend instead of types"""
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
    legend=:outertopright,
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
    source::Vector{String}
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

# Prepare a group for the data
group = LEGEND_SHOW_SOURCES ? data.source : data.obj_type

println(" "^pad, "> Plotting the scatter plots...")

# Plot a scatter plot in the (X, Y) plane
scatter(
    data.x,
    data.y;
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Y \; \mathrm{[kpc]}",
    group,
)
savefig(joinpath(PLOTS_DIR, "XY$(POSTFIX).pdf"))

# Plot a scatter plot in the (X, Z) plane
scatter(
    data.x,
    data.z;
    xlabel=L"X \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    group,
)
savefig(joinpath(PLOTS_DIR, "XZ$(POSTFIX).pdf"))

# Plot a scatter plot in the (Y, Z) plane
scatter(
    data.y,
    data.z;
    xlabel=L"Y \; \mathrm{[kpc]}",
    ylabel=L"Z \; \mathrm{[kpc]}",
    group,
)
savefig(joinpath(PLOTS_DIR, "YZ$(POSTFIX).pdf"))

println()
