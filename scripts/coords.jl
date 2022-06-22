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
function parse_string(i)::String
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
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
    # Show sources on the legend instead of types
    if ARGS[i] == "-s"
        check_last(i)
        global LEGEND_SHOW_SOURCES = true
    end
    # Output directory
    if ARGS[i] == "-o"
        try
            global OUTPUT_DIR = parse_string(i+1)
        catch
            println("Couldn't parse the value of the `-o` argument.")
            exit(1)
        end
    end
    # A postfix for the names of output files
    if ARGS[i] == "--postfix"
        try
            global POSTFIX = " ($(parse_string(i+1)))"
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
        { julia --project=. | ./julia.bash } scripts/coords.jl [-s] [-o <OUTPUT_DIR>] [--postfix <POSTFIX>] <INPUT_DIR>

        $(YELLOW)ARGS:$(RESET)
        $(GREEN)<INPUT_DIR>$(RESET)    Input directory (relative to the root of the repository)

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-s$(RESET)                     Show sources on the legend instead of types
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory (relative to the root of the repository)
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files"""
    )
    exit(1)
end

# Define the input directory
INPUT_DIR = ARGS[end]

"Padding in the output"
pad = 4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = UInt64

println('\n', " "^pad, "> Loading the packages...")

using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Choose a color scheme
cmap = ColorSchemes.tol_light[2:end]

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
PLOTS_DIR = joinpath(ROOT_DIR, "plots", OUTPUT_DIR)
DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "bin", "coords.bin")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the data files
println(" "^pad, "> Loading the data...")

struct Data
    names::Vector{String}
    l::Vector{F}
    b::Vector{F}
    r::Vector{F}
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
data = read_bincode(DATA_PATH)

# Prepare a group for the data
group = LEGEND_SHOW_SOURCES ? data.source : data.obj_type

# Prepare labels
markers = ["a", "b", "c", "d", "e", "g"]
keys = unique(group)
dictionary = Dict([ (k, markers[i]) for (i, k) in enumerate(keys) ])
label = [ dictionary[k] for k in group ]

println(" "^pad, "> Plotting the scatter plots...")

"Create a scatter plot"
function scatter(x, y, xlabel, ylabel)
    return @pgf Axis(
        {
            xlabel = xlabel,
            ylabel = ylabel,
            height = 200,
            width = 200,
            grid = "both",
            minor_tick_num = 5,
            minor_grid_style = { opacity = 0.25 },
            major_grid_style = { opacity = 0.5 },
            tick_label_style = { font = "\\small" },
            tick_style = { line_width = 0.4, color = "black" },
            axis_line_style = { line_width = 1 },
            "axis_lines*" = "left",
            legend_image_post_style = { mark_size = 2, line_width = 0.4 },
            legend_pos = "outer north east",
            legend_style = { line_width = 1 },
            mark_size = 0.5,
            line_width = 0.15,
            "scatter/classes" = {
                a = { mark = "x", color = cmap[1] },
                b = { mark = "+", color = cmap[2] },
                c = { mark = "asterisk", color = cmap[3] },
                d = { mark = "star", color = cmap[4] },
                e = { mark = "10-pointed star", color = cmap[5] },
            },
        },
        Plot(
            {
                scatter,
                "only marks",
                "scatter src" = "explicit symbolic",
            },
            Table(
                {
                    meta = "label",
                },
                x = x,
                y = y,
                label = label,
            ),
        ),
        Legend(keys),
    )
end

# Plot a scatter plot in the (X, Y) plane
p = scatter(data.x, data.y, L"X \; \mathrm{[kpc]}", L"Y \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "XY$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane
p = scatter(data.x, data.z, L"X \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "XZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane
p = scatter(data.y, data.z, L"Y \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "YZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane
p = scatter(data.r, data.z, L"R \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "RZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (l, b) plane
p = scatter(data.l, data.b, L"l \; \mathrm{[deg]}", L"b \; \mathrm{[deg]}")
pgfsave(joinpath(PLOTS_DIR, "lb$(POSTFIX).pdf"), p)

println()
