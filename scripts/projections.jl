# This script plots the projections in each plane

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
    else
        # Return the next argument
        ARGS[i]
    end
    return s
end

# Define default values for the arguments
LEGEND_SHOW_SOURCES = false
INPUT_DIR = ""
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
    # Show sources on the legend instead of types
    if ARGS[i] == "-s"
        global LEGEND_SHOW_SOURCES = true
    end
    # Input directory
    if ARGS[i] == "-i"
        try
            global INPUT_DIR = parse_string(i+1)
        catch
            println("Couldn't parse the value of the `-i` argument.")
            exit(1)
        end
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

# Show help if requested
if "--help" in ARGS
    println("""
        $(YELLOW)USAGE:$(RESET)
        { julia --project=. | ./julia.bash } scripts/projections.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [-s] [--postfix <POSTFIX>]

        $(YELLOW)ARGS:$(RESET)
        $(GREEN)<INPUT_DIR>$(RESET)    Input directory (relative to the root of the repository)

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-s$(RESET)                     Show sources on the legend instead of types
            $(GREEN)-o <INPUT_DIR>$(RESET)         Input directory
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files"""
    )
    exit(1)
end

# Make sure the required arguments are passed
if isempty(INPUT_DIR)
    println("An input file is required.")
    exit(1)
end
if isempty(OUTPUT_DIR)
    println("An output directory is required.")
    exit(1)
end

"Padding in the output"
pad = " "^4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = UInt64

println('\n', pad, "> Loading the packages...")

using Base.Threads
using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Choose a color scheme
colors = ColorSchemes.tol_bright[2:end]

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? OUTPUT_DIR : joinpath(ROOT_DIR, OUTPUT_DIR)
DATA_PATH = joinpath(INPUT_DIR, "objects.bin")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the paths to the data files
println(pad, "> Loading the data...")

include(joinpath(CURRENT_DIR, "data_types.jl"))

Data = Types.ObjectsData{F}

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
counts = Dict([(k, count(==(k), group)) for k in keys])
I = sortperm(group, by=k -> counts[k], rev=true)
group = group[I]
l = data.l[I]
b = data.b[I]
X = data.X[I]
X_p = data.X_p[I]
X_m = data.X_m[I]
Y = data.Y[I]
Y_p = data.Y_p[I]
Y_m = data.Y_m[I]
Z = data.Z[I]
Z_p = data.Z_p[I]
Z_m = data.Z_m[I]
R = data.R[I]
R_p = data.R_p[I]
R_m = data.R_m[I]

# Prepare labels
labels = ["a", "b", "c", "d", "e", "g"]
dictionary = Dict([(k, labels[i]) for (i, k) in enumerate(keys)])
label = [dictionary[k] for k in group]

println(pad, "> Plotting the projections...")

"Compute the limits from the collection"
function max_min(c; factor=0.1)
    max_value = maximum(c)
    min_value = minimum(c)
    len = max_value - min_value
    max_value = max_value + factor * len
    min_value = min_value - factor * len
    return max_value, min_value
end

"Create a scatter plot"
function scatter(
    x,
    y,
    xlabel,
    ylabel;
    x_p=F[],
    x_m=F[],
    y_p=F[],
    y_m=F[],
    axis_equal=false,
    crosses=false,
    x_is_positive=false,
)
    # Compute the limits
    x_max, x_min = max_min(x)
    y_max, y_min = max_min(y)
    if x_is_positive
        x_min = max(0, x_min)
    end
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
            minor_grid_style = {opacity = 0.25},
            major_grid_style = {opacity = 0.5},
            tick_label_style = {font = "\\small"},
            tick_style = {line_width = 0.4, color = "black"},
            axis_equal = axis_equal,
            axis_on_top = true,
            axis_line_style = {line_width = 1},
            "axis_lines*" = "left",
            legend_image_post_style = {mark_size = 2, line_width = 0.4},
            legend_pos = "outer north east",
            legend_style = {line_width = 1},
            mark_size = 0.5,
            line_width = 0.15,
            "scatter/classes" = {
                a = {mark = marks[1], color = colors[1]},
                b = {mark = marks[2], color = colors[2]},
                c = {mark = marks[3], color = colors[3]},
                d = {mark = marks[4], color = colors[4]},
                e = {mark = marks[5], color = colors[5]},
            },
        },
        Plot(
            {
                scatter,
                only_marks,
                scatter_src = "explicit symbolic",
            },
            Table(
                {
                    meta = "label",
                },
                x=x,
                y=y,
                label=label,
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
                    line_width = 0.1,
                },
                Coordinates([(x_m, y_m), (x, y), (x_p, y_p)])
            ))
        end
    end
    return p
end

print_lock = ReentrantLock()
tasks = Task[]

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XY...")
    end
    p = scatter(
        X,
        Y,
        L"X \; \mathrm{[kpc]}",
        L"Y \; \mathrm{[kpc]}",
        axis_equal=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "XY$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XY (errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "XY (errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XY (crosses)...")
    end
    p = scatter(
        X,
        Y,
        L"X \; \mathrm{[kpc]}",
        L"Y \; \mathrm{[kpc]}",
        axis_equal=true,
        crosses=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "XY (crosses)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XY (crosses, errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "XY (crosses, errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XZ...")
    end
    p = scatter(
        X,
        Z,
        L"X \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "XZ$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XZ (equal axes)...")
    end
    p = scatter(
        X,
        Z,
        L"X \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
        axis_equal=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "XZ (equal axes)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XZ (errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "XZ (errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for XZ (equal axes, errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "XZ (equal axes, errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for YZ...")
    end
    p = scatter(
        Y,
        Z,
        L"Y \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "YZ$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for YZ (equal axes)...")
    end
    p = scatter(
        Y,
        Z,
        L"Y \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
        axis_equal=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "YZ (equal axes)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for YZ (errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "YZ (errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for YZ (equal axes, errors)...")
    end
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
    pgfsave(joinpath(OUTPUT_DIR, "YZ (equal axes, errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for RZ...")
    end
    p = scatter(
        R,
        Z,
        L"R \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
        x_is_positive=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "RZ$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for RZ (equal axes)...")
    end
    p = scatter(
        R,
        Z,
        L"R \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
        axis_equal=true,
        x_is_positive=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "RZ (equal axes)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for RZ (errors)...")
    end
    p = scatter(
        R,
        Z,
        L"R \; \mathrm{[kpc]}",
        L"Z \; \mathrm{[kpc]}",
        x_p=R_p,
        x_m=R_m,
        y_p=Z_p,
        y_m=Z_m,
        x_is_positive=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "RZ (errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for RZ (equal axes, errors)...")
    end
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
        x_is_positive=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "RZ (equal axes, errors)$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for lb...")
    end
    p = scatter(
        l,
        b,
        L"l \; \mathrm{[deg]}",
        L"b \; \mathrm{[deg]}",
        axis_equal=true,
    )
    pgfsave(joinpath(OUTPUT_DIR, "lb$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

# Mark data for garbage collection
data = nothing

println()
