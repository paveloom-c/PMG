# This script plots the rotation curves with
# initial parameters and fitted parameters

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
        { julia --project=. | ./julia.bash } scripts/fit_rotcurve.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [-s] [--postfix <POSTFIX>]

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
colors = ColorSchemes.tol_bright

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? OUTPUT_DIR : joinpath(ROOT_DIR, OUTPUT_DIR)
OBJECTS_DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "fit_objects.bin")
FIT_ROTCURVE_DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "fit_rotcurve.bin")
FIT_PARAMS_DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "fit_params.bin")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the paths to the data files
println(pad, "> Loading the data...")

include(joinpath(CURRENT_DIR, "data_types.jl"))

ObjectsData = Types.ObjectsData{F}
FitRotCurveData = Types.FitRotCurveData{F}
FitParamsData = Types.Params{F}

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString, type::Type)::type
    open(path, "r") do io
        # Read the number of objects
        n = read(io, I)
        # Get the fields and their types
        fields = fieldnames(type)
        types = eltype.(fieldtypes(type))
        # Initialize the data struct
        data = type(ntuple(_ -> [], length(fields))...)
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
objects_data = read_bincode(OBJECTS_DATA_PATH, ObjectsData)
fit_rotcurve_data = read_bincode(FIT_ROTCURVE_DATA_PATH, FitRotCurveData)
fit_params_data = read_bincode(FIT_PARAMS_DATA_PATH, FitParamsData)

# Prepare a group for the data
group = LEGEND_SHOW_SOURCES ? objects_data.source : objects_data.type

# Sort the data by the number of occurrences of different types
# (rare types will be plotted over common types)
keys = unique(group)
counts = Dict([(k, count(==(k), group)) for k in keys])
I = sortperm(group, by=k -> counts[k], rev=true)
group = group[I]
R = objects_data.R[I]
R_p = objects_data.R_p[I]
R_m = objects_data.R_m[I]
Θ = objects_data.Θ[I]
Θ_p = objects_data.Θ_p[I]
Θ_m = objects_data.Θ_m[I]
Θ_evel = objects_data.Θ_evel[I]

# Unpack the fit data
R_fit = fit_rotcurve_data.R
Θ_fit = fit_rotcurve_data.Θ
R_0 = fit_params_data.R_0[1]
θ_sun = fit_params_data.θ_sun[1]

# Prepare labels
markers = ["a", "b", "c", "d", "e", "g"]
dictionary = Dict([(k, markers[i]) for (i, k) in enumerate(keys)])
label = [dictionary[k] for k in group]

println(pad, "> Plotting the fitted rotation curves...")

"Compute the limits from the collection"
function max_min(c; factor=0.1)
    max_value = maximum(c)
    min_value = minimum(c)
    len = max_value - min_value
    max_value = max_value + factor * len
    min_value = max(0, min_value - factor * len)
    return max_value, min_value
end

function plot(
    x,
    y,
    x_fit,
    y_fit,
    xlabel,
    ylabel;
    x_p=F[],
    x_m=F[],
    y_p=F[],
    y_m=F[],
    evel=F[],
)
    # Compute the limits
    x_max, x_min = max_min(x)
    y_max, y_min = max_min(y)
    # Define the X tick distance
    xtick_distance = (x_max - x_min) > 10 ? 4 : 2
    # Prepare tables
    objects_table =
        if isempty(evel)
            @pgf Table(
                {
                    meta = "label",
                },
                x=x,
                y=y,
                label=label,
            )
        else
            @pgf Table(
                {
                    meta = "label",
                    y_error_plus = "evel",
                    y_error_minus = "evel",
                },
                x=x,
                y=y,
                label=label,
                evel=evel,
            )
        end
    fit_table = @pgf Table(
        x=x_fit,
        y=y_fit,
    )
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
            xtick_distance = xtick_distance,
            minor_x_tick_num = 3,
            minor_y_tick_num = 4,
            minor_grid_style = {opacity = 0.25},
            major_grid_style = {opacity = 0.5},
            tick_label_style = {font = "\\small"},
            tick_style = {line_width = 0.4, color = "black"},
            "error bars/error bar style" = {line_width = 0.1, opacity = 0.25},
            "error bars/error mark options" = {
                rotate = 90,
                mark_size = 0.5,
                line_width = 0.1,
                opacity = 0.25,
            },
            axis_line_style = {line_width = 1},
            axis_on_top = true,
            "axis_lines*" = "left",
            legend_image_post_style = {mark_size = 2, line_width = 0.4},
            legend_pos = "outer north east",
            legend_style = {line_width = 1},
            mark_size = 0.5,
            line_width = 0.15,
            "scatter/classes" = {
                a = {mark = "x", color = colors[2]},
                b = {mark = "+", color = colors[3]},
                c = {mark = "asterisk", color = colors[4]},
                d = {mark = "star", color = colors[5]},
                e = {mark = "10-pointed star", color = colors[6]},
            },
        },
        Plot(
            {
                scatter,
                only_marks,
                scatter_src = "explicit symbolic",
                "error bars/y dir=both",
                "error bars/y explicit",
            },
            objects_table,
        ),
        Plot(
            {
                color = colors[1],
                mark = "none"
            },
            fit_table,
        ),
        Plot(
            {
                scatter,
                color = "black",
                mark_size = 0.4,
            },
            Coordinates([R_0], [θ_sun])
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
        println(pad, pad, "without errors...")
    end
    p = plot(
        R,
        Θ,
        R_fit,
        Θ_fit,
        L"R \; \mathrm{[kpc]}",
        L"\theta \; \mathrm{[km \; s^{-1}]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "Fitted rotation curve$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "with errors...")
    end
    p = plot(
        R,
        Θ,
        R_fit,
        Θ_fit,
        L"R \; \mathrm{[kpc]}",
        L"\theta \; \mathrm{[km \; s^{-1}]}",
        x_p=R_p,
        x_m=R_m,
        y_p=Θ_p,
        y_m=Θ_m,
        evel=Θ_evel,
    )
    pgfsave(joinpath(OUTPUT_DIR, "Fitted rotation curve (errors)$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        throw(err.task.exception)
    end
end

# Mark data for garbage collection
objects_data = nothing
fit_rotcurve_data = nothing
fit_params_data = nothing

println()
