# This script plots the difference between the
# original parallaxes and the reduced ones

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
N = nothing
INPUT_DIR = ""
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
    # Degree of the model
    if ARGS[i] == "-n"
        try
            global N = parse(Int, ARGS[i+1])
        catch
            println("Couldn't parse the value of the `-n` argument.")
            exit(1)
        end
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
        { julia --project=. | ./julia.bash } scripts/parallaxes.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [--postfix <POSTFIX>]

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-n$(RESET)                     Degree of the polynomial of the rotation curve
            $(GREEN)-i <INPUT_DIR>$(RESET)         Input directory
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files"""
    )
    exit(1)
end

# Make sure the required arguments are passed
if isnothing(N)
    println("A degree of the polynomial is required.")
    exit(1)
end
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

using CSV
using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Choose a color scheme
colors = ColorSchemes.tol_bright[2:end]

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
INPUT_MODEL_DIR = joinpath(INPUT_DIR, "n = $(N)")
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? joinpath(OUTPUT_DIR, "n = $(N)") : joinpath(ROOT_DIR, OUTPUT_DIR, "n = $(N)")
DELTA_VARPI_DATA_PATH = joinpath(INPUT_DIR, "Delta_varpi.dat")
PARALLAXES_DATA_PATH = joinpath(INPUT_MODEL_DIR, "parallaxes.dat")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

println(pad, "> Loading the data...")

# Read the parallaxes data
parallaxes_data = CSV.File(PARALLAXES_DATA_PATH, delim=' ', comment="#")
delta_varpi_data = CSV.File(DELTA_VARPI_DATA_PATH, delim=' ', comment="#")

x_mean = delta_varpi_data.x_mean[N]
sigma_x_mean = delta_varpi_data.sigma_x_mean[N]
sigma_stroke = delta_varpi_data.sigma_stroke[N]

# Prepare a group for the data
group = parallaxes_data.source

# Sort the data by the number of occurrences of different types
# (rare types will be plotted over common types)
keys = unique(group)
counts = Dict([(k, count(==(k), group)) for k in keys])
I = sortperm(group, by=k -> counts[k], rev=true)
group = group[I]
par = parallaxes_data.par[I]
par_e = parallaxes_data.par_e[I]
par_r = parallaxes_data.par_r[I]

# Prepare labels
labels = ["a", "b", "c", "d", "e", "g"]
dictionary = Dict([(k, labels[i]) for (i, k) in enumerate(keys)])
label = [dictionary[k] for k in group]

println(pad, "> Plotting the parallaxes...")

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
    x_e,
    y,
    xlabel,
    ylabel,
)
    # Compute the limits
    x_max, x_min = (1, 0)
    y_max, y_min = (1, 0)
    # Define the markers set
    marks = ["x", "+", "asterisk", "star", "10-pointed star"]
    # Create a plot
    return @pgf TikzPicture(
        Axis(
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
                axis_equal = true,
                axis_on_top = true,
                axis_line_style = {line_width = 1},
                "axis_lines*" = "left",
                legend_image_post_style = {mark_size = 2, line_width = 0.4},
                legend_pos = "outer north east",
                legend_style = { line_width = 1, name = "leg" },
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
                    "error bars/x dir=both",
                    "error bars/x explicit",
                },
                Table(
                    {
                        meta = "label",
                        x_error = "x_e",
                    },
                    x=x,
                    y=y,
                    label=label,
                    x_e=x_e,
                ),
            ),
            Legend(keys),
            Plot({ no_marks, opacity = 0.75}, Expression("x")),
            [raw"\label{bisector}"],
            Plot({ no_marks, opacity = 0.75, color = colors[3]}, Expression("x + $(x_mean)")),
            [raw"\label{x_mean}"],
            Plot({ no_marks, opacity = 0.75, color = colors[2]}, Expression("x + $(x_mean) + $(sigma_x_mean)")),
            [raw"\label{sigma_x_mean}"],
            Plot({ no_marks, opacity = 0.75, color = colors[2]}, Expression("x + $(x_mean) - $(sigma_x_mean)")),
            Plot({ no_marks, opacity = 0.75, color = colors[1]}, Expression("x + $(x_mean) + $(sigma_stroke)")),
            [raw"\label{sigma_stroke}"],
            Plot({ no_marks, opacity = 0.75, color = colors[1]}, Expression("x + $(x_mean) - $(sigma_stroke)")),
        ),
        raw"""
        \node [draw,fill=white,line width=1,below,outer sep=1cm] at (leg) {\shortstack[l]{
            \ref{bisector} bis \\
            \ref{x_mean} $ \overline{x} $ \\
            \ref{sigma_x_mean} $ \sigma_{\overline{x}} $ \\
            \ref{sigma_stroke} $ \sigma' $}};""",
    )
end

p = scatter(
    par,
    par_e,
    par_r,
    L"\varpi",
    L"\varpi_0",
)
pgfsave(joinpath(OUTPUT_DIR, "Parallaxes$(POSTFIX).pdf"), p)

# Mark data for garbage collection
data = nothing

println()
