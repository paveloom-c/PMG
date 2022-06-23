# This script plots the projections in each plane

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
pad = " "^4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = UInt64

println('\n', pad, "> Loading the packages...")

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
println(pad, "> Loading the data...")

struct Data
    name::Vector{String}
    l::Vector{F}
    b::Vector{F}
    X::Vector{F}
    ep_X::Vector{F}
    em_X::Vector{F}
    Y::Vector{F}
    ep_Y::Vector{F}
    em_Y::Vector{F}
    Z::Vector{F}
    ep_Z::Vector{F}
    em_Z::Vector{F}
    r::Vector{F}
    ep_r::Vector{F}
    em_r::Vector{F}
    R::Vector{F}
    ep_R::Vector{F}
    em_R::Vector{F}
    type::Vector{String}
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
group = LEGEND_SHOW_SOURCES ? data.source : data.type

# Prepare labels
markers = ["a", "b", "c", "d", "e", "g"]
keys = unique(group)
dictionary = Dict([ (k, markers[i]) for (i, k) in enumerate(keys) ])
label = [ dictionary[k] for k in group ]

println(pad, "> Plotting the scatter plots...")

"Create a scatter plot"
function scatter(x, y, xlabel, ylabel; ep_x = F[], em_x = F[], ep_y = F[], em_y = F[], axis_equal=false)
    table = if isempty(ep_x) && isempty(em_x) && isempty(ep_y) && isempty(em_y)
        @pgf Table(
            {
                meta = "label",
            },
            x = x,
            y = y,
            label = label,
        )
    else
        @pgf Table(
            {
                meta = "label",
                x_error_plus = "ep_x",
                x_error_minus = "em_x",
                y_error_plus = "ep_y",
                y_error_minus = "em_y",
            },
            x = x,
            y = y,
            label = label,
            ep_x = ep_x,
            em_x = em_x,
            ep_y = ep_y,
            em_y = em_y,
        )
    end
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
            "error bars/error bar style" = { line_width = 0.1, opacity = 0.25 },
            "error bars/error mark options" = {
                rotate = 90,
                mark_size = 0.5,
                line_width = 0.1,
                opacity = 0.25,
            },
            axis_equal = axis_equal,
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
                "error bars/x dir=both",
                "error bars/y dir=both",
                "error bars/x explicit",
                "error bars/y explicit",
            },
            table,
        ),
        Legend(keys),
    )
end

# Plot a scatter plot in the (X, Y) plane
println(pad, "    for XY...")
p = scatter(
    data.X,
    data.Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}";
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Y) plane with errors
println(pad, "    for XY (errors)...")
p = scatter(
    data.X,
    data.Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}";
    ep_x=data.ep_X,
    em_x=data.em_X,
    ep_y=data.ep_Y,
    em_y=data.em_Y,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane
println(pad, "    for XZ...")
p = scatter(data.X, data.Z, L"X \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "XZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with equal axes
println(pad, "    for XZ (equal axes)...")
p = scatter(
    data.X,
    data.Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}";
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with errors
println(pad, "    for XZ (errors)...")
p = scatter(
    data.X,
    data.Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}";
    ep_x=data.ep_X,
    em_x=data.em_X,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with errors and equal axes
println(pad, "    for XZ (equal axes, errors)...")
p = scatter(
    data.X,
    data.Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}";
    ep_x=data.ep_X,
    em_x=data.em_X,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane
println(pad, "    for YZ...")
p = scatter( data.Y, data.Z, L"Y \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "YZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with equal axes
p = scatter(
    data.Y,
    data.Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with errors
println(pad, "    for YZ (errors)...")
p = scatter(
    data.Y,
    data.Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    ep_x=data.ep_Y,
    em_x=data.em_Y,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with errors and equal axes
println(pad, "    for YZ (equal axes, errors)...")
p = scatter(
    data.Y,
    data.Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    ep_x=data.ep_Y,
    em_x=data.em_Y,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane
println(pad, "    for RZ...")
p = scatter(data.R, data.Z, L"R \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "RZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with equal axes
println(pad, "    for RZ (equal axes)...")
p = scatter(
    data.R,
    data.Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with errors
println(pad, "    for RZ (errors)...")
p = scatter(
    data.R,
    data.Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    ep_x=data.ep_R,
    em_x=data.em_R,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with errors and equal axes
println(pad, "    for RZ (equal axes, errors)...")
p = scatter(
    data.R,
    data.Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    ep_x=data.ep_R,
    em_x=data.em_R,
    ep_y=data.ep_Z,
    em_y=data.em_Z,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (l, b) plane
println(pad, "    for lb...")
p = scatter(
    data.l,
    data.b,
    L"l \; \mathrm{[deg]}",
    L"b \; \mathrm{[deg]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "lb$(POSTFIX).pdf"), p)

println()
