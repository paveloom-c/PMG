# This script plots the rotation curves with
# initial parameters and fitted parameters

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
            global OUTPUT_DIR = parse_string(i + 1)
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
if length(ARGS) <= 1 || "--help" in ARGS
    println("""
        $(YELLOW)USAGE:$(RESET)
        { julia --project=. | ./julia.bash } scripts/fit_rotcurve.jl [-s] [-o <OUTPUT_DIR>] [--postfix <POSTFIX>] <INPUT_DIR>

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

using Base.Threads
using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Choose a color scheme
colors = ColorSchemes.tol_bright

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
PLOTS_DIR = joinpath(ROOT_DIR, "plots", OUTPUT_DIR)
OBSERVED_DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "bin", "objects.bin")
FIT_DATA_PATH = joinpath(ROOT_DIR, INPUT_DIR, "bin", "fit_rotcurve.bin")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the data files
println(pad, "> Loading the data...")

struct ObservedData
    name::Vector{String}
    type::Vector{String}
    source::Vector{String}
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
    mu_l::Vector{F}
    mu_b::Vector{F}
    V_r::Vector{F}
    V_l::Vector{F}
    ep_V_l::Vector{F}
    em_V_l::Vector{F}
    V_b::Vector{F}
    ep_V_b::Vector{F}
    em_V_b::Vector{F}
    U::Vector{F}
    ep_U::Vector{F}
    em_U::Vector{F}
    V::Vector{F}
    ep_V::Vector{F}
    em_V::Vector{F}
    W::Vector{F}
    ep_W::Vector{F}
    em_W::Vector{F}
    Θ::Vector{F}
    ep_Θ::Vector{F}
    em_Θ::Vector{F}
    evel_Θ::Vector{F}
end

struct FitData
    R::Vector{F}
    Θ::Vector{F}
end

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
observed_data = read_bincode(OBSERVED_DATA_PATH, ObservedData)
fit_data = read_bincode(FIT_DATA_PATH, FitData)

# Prepare a group for the data
group = LEGEND_SHOW_SOURCES ? observed_data.source : observed_data.type

# Sort the data by the number of occurrences of different types
# (rare types will be plotted over common types)
keys = unique(group)
counts = Dict([(k, count(==(k), group)) for k in keys])
I = sortperm(group, by=k -> counts[k], rev=true)
group = group[I]
R = observed_data.R[I]
ep_R = observed_data.ep_R[I]
em_R = observed_data.em_R[I]
Θ = observed_data.Θ[I]
ep_Θ = observed_data.ep_Θ[I]
em_Θ = observed_data.em_Θ[I]
evel_Θ = observed_data.evel_Θ[I]

# Compute the secondary data sets
R_p = R .+ ep_R
R_m = R .- em_R
Θ_p = Θ .+ ep_Θ
Θ_m = Θ .- em_Θ

# Unpack the fit data
R_fit = fit_data.R
Θ_fit = fit_data.Θ

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
    # Prepare a table for the observed
    # values of the rotation curve
    observed_table =
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
    # Prepare a table for the fit
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
            minor_tick_num = 4,
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
            observed_table,
        ),
        Plot(
            {
                color = colors[1],
                mark = "none"
            },
            fit_table,
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
    pgfsave(joinpath(PLOTS_DIR, "Fit rotation curve$(POSTFIX).pdf"), p)
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
        evel=evel_Θ,
    )
    pgfsave(joinpath(PLOTS_DIR, "Fit rotation curve (errors)$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        throw(err.task.exception)
    end
end

# Mark data for garbage collection
observed_data = nothing
fit_data = nothing

println()
