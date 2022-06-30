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
colors = ColorSchemes.tol_light[2:end]

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

# Sort the data by the number of occurrences of different types
# (rare types will be plotted over common types)
keys = unique(group)
counts = Dict([ (k, count(==(k), group)) for k in keys ])
I = sortperm(group, by=k->counts[k], rev=true)
group = group[I]
l = data.l[I]
b = data.b[I]
X = data.X[I]
Y = data.Y[I]
Z = data.Z[I]
R = data.R[I]
ep_X = data.ep_X[I]
ep_Y = data.ep_Y[I]
ep_Z = data.ep_Z[I]
ep_R = data.ep_R[I]
em_X = data.em_X[I]
em_Y = data.em_Y[I]
em_Z = data.em_Z[I]
em_R = data.em_R[I]

# Compute the secondary data sets
X_p = X .+ ep_X
Y_p = Y .+ ep_Y
Z_p = Z .+ ep_Z
R_p = R .+ ep_R
X_m = X .- em_X
Y_m = Y .- em_Y
Z_m = Z .- em_Z
R_m = R .- em_R

# Prepare labels
labels = ["a", "b", "c", "d", "e", "g"]
dictionary = Dict([ (k, labels[i]) for (i, k) in enumerate(keys) ])
label = [ dictionary[k] for k in group ]

println(pad, "> Plotting the scatter plots...")

"Compute the limits from the collection"
function max_min(c; factor=0.1)
    max = maximum(c)
    min = minimum(c)
    len = max - min
    max = max + factor * len
    min = min - factor * len
    return max, min
end

"Create a scatter plot"
function scatter(x, y, xlabel, ylabel; x_p = F[], x_m = F[], y_p = F[], y_m = F[], axis_equal=false, crosses=false)
    # Compute the limits
    x_max, x_min = max_min(x)
    y_max, y_min = max_min(y)
    # Define the markers set
    marks = if crosses
        ["x", "+", "asterisk", "star", "10-pointed star"]
    else
        repeat(["*"], 5)
    end
    # Create a plot
    p = @pgf Axis(
        {
            xlabel = xlabel,
            ylabel = ylabel,
            xmax = x_max,
            xmin = x_min,
            ymax = y_max,
            ymin = y_min,
            height = 200,
            width = 200,
            grid = "both",
            minor_tick_num = 4,
            minor_grid_style = { opacity = 0.25 },
            major_grid_style = { opacity = 0.5 },
            tick_label_style = { font = "\\small" },
            tick_style = { line_width = 0.4, color = "black" },
            axis_equal = axis_equal,
            axis_line_style = { line_width = 1 },
            "axis_lines*" = "left",
            legend_image_post_style = { mark_size = 2, line_width = 0.4 },
            legend_pos = "outer north east",
            legend_style = { line_width = 1 },
            mark_size = 0.5,
            line_width = 0.15,
            "scatter/classes" = {
                a = { mark = marks[1], color = colors[1] },
                b = { mark = marks[2], color = colors[2] },
                c = { mark = marks[3], color = colors[3] },
                d = { mark = marks[4], color = colors[4] },
                e = { mark = marks[5], color = colors[5] },
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
    # Add the error lines if additional data sets are specified
    if !isempty(x_p) && !isempty(x_m) && !isempty(y_p) && !isempty(y_m)
        for (x, y, x_p, x_m, y_p, y_m) in zip(x, y, x_p, x_m, y_p, y_m)
            push!(p, @pgf Plot(
                {
                    no_marks,
                    opacity = 0.25,
                },
                Coordinates([(x_m, y_m), (x, y), (x_p, y_p)]),
            ))
        end
    end
    return p
end

# Plot a scatter plot in the (X, Y) plane
println(pad, "    for XY...")
p = scatter(
    X,
    Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Y) plane with errors
println(pad, "    for XY (errors)...")
p = scatter(
    X,
    Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}",
    x_p=X_p,
    x_m=X_m,
    y_p=Y_p,
    y_m=Y_m,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Y) plane with crosses
println(pad, "    for XY (crosses)...")
p = scatter(
    X,
    Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}",
    axis_equal=true,
    crosses=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY (crosses)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Y) plane with crosses and errors
println(pad, "    for XY (crosses, errors)...")
p = scatter(
    X,
    Y,
    L"X \; \mathrm{[kpc]}",
    L"Y \; \mathrm{[kpc]}",
    x_p=X_p,
    x_m=X_m,
    y_p=Y_p,
    y_m=Y_m,
    axis_equal=true,
    crosses=true,
)
pgfsave(joinpath(PLOTS_DIR, "XY (crosses, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane
println(pad, "    for XZ...")
p = scatter(X, Z, L"X \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "XZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with equal axes
println(pad, "    for XZ (equal axes)...")
p = scatter(
    X,
    Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with errors
println(pad, "    for XZ (errors)...")
p = scatter(
    X,
    Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=X_p,
    x_m=X_m,
    y_p=Z_p,
    y_m=Z_m,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (X, Z) plane with errors and equal axes
println(pad, "    for XZ (equal axes, errors)...")
p = scatter(
    X,
    Z,
    L"X \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=X_p,
    x_m=X_m,
    y_p=Z_p,
    y_m=Z_m,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "XZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane
println(pad, "    for YZ...")
p = scatter(Y, Z, L"Y \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "YZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with equal axes
p = scatter(
    Y,
    Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with errors
println(pad, "    for YZ (errors)...")
p = scatter(
    Y,
    Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=Y_p,
    x_m=Y_m,
    y_p=Z_p,
    y_m=Z_m,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (Y, Z) plane with errors and equal axes
println(pad, "    for YZ (equal axes, errors)...")
p = scatter(
    Y,
    Z,
    L"Y \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=Y_p,
    x_m=Y_m,
    y_p=Z_p,
    y_m=Z_m,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "YZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane
println(pad, "    for RZ...")
p = scatter(R, Z, L"R \; \mathrm{[kpc]}", L"Z \; \mathrm{[kpc]}")
pgfsave(joinpath(PLOTS_DIR, "RZ$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with equal axes
println(pad, "    for RZ (equal axes)...")
p = scatter(
    R,
    Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (equal axes)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with errors
println(pad, "    for RZ (errors)...")
p = scatter(
    R,
    Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=R_p,
    x_m=R_m,
    y_p=Z_p,
    y_m=Z_m,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (R, Z) plane with errors and equal axes
println(pad, "    for RZ (equal axes, errors)...")
p = scatter(
    R,
    Z,
    L"R \; \mathrm{[kpc]}",
    L"Z \; \mathrm{[kpc]}",
    x_p=R_p,
    x_m=R_m,
    y_p=Z_p,
    y_m=Z_m,
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "RZ (equal axes, errors)$(POSTFIX).pdf"), p)

# Plot a scatter plot in the (l, b) plane
println(pad, "    for lb...")
p = scatter(
    l,
    b,
    L"l \; \mathrm{[deg]}",
    L"b \; \mathrm{[deg]}",
    axis_equal=true,
)
pgfsave(joinpath(PLOTS_DIR, "lb$(POSTFIX).pdf"), p)

println()
