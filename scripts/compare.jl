# This script plots the comparison charts for the objects that
# are identified as the same in the first VERA catalogue (2020)
# and the catalogue from Reid et al. (2019)

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
end

# Define default values for optional arguments
LEGEND_SHOW_SOURCES = false
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
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
if "--help" in ARGS
    println("""
        $(YELLOW)USAGE:$(RESET)
        { julia --project=. | ./julia.bash } scripts/compare.jl [-s] [-o <OUTPUT_DIR>] [--postfix <POSTFIX>]

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory (relative to the root of the repository)
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files"""
    )
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

# Define the markers set
marks = repeat(["*"], 5)

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
PLOTS_DIR = joinpath(ROOT_DIR, "plots", OUTPUT_DIR)
INPUT = joinpath(ROOT_DIR, "data", "input", "vera_reid.dat")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the data files
println(pad, "> Loading the data...")

# Read the data
data = CSV.File(INPUT, delim=' ', comment="#")
e_μ_x = data.e_mu_x
e_μ_y = data.e_mu_y
e_v_lsr = data.e_v_lsr

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
function scatter(x, y, xlabel, ylabel)
    # Compute the limits
    x_max, x_min = max_min(x)
    y_max, y_min = max_min(y)
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
            axis_line_style = { line_width = 1 },
            "axis_lines*" = "left",
            mark = "*",
            mark_size = 0.5,
            "scatter/use mapped color" = {
                draw_opacity = 0,
                fill = "mapped color",
            },
        },
        Plot(
            {
                scatter,
                "only marks",
            },
            Coordinates(x, y),
        ),
    )
    return p
end

# Plot a scatter plot for the Eastward proper motions
println(pad, "    for mu_x...")
p = scatter(
    e_μ_x[1:2:end],
    e_μ_x[2:2:end],
    L"\sigma_{\mu_x} \; \mathrm{[mas \; yr^{-1}]} \; \mathrm{[VERA]}",
    L"\sigma_{\mu_x} \; \mathrm{[mas \; yr^{-1}]} \; \mathrm{[Reid]}",
)
pgfsave(joinpath(PLOTS_DIR, "mu_x$(POSTFIX).pdf"), p)

# Plot a scatter plot for the Northward proper motions
println(pad, "    for mu_y...")
p = scatter(
    e_μ_y[1:2:end],
    e_μ_y[2:2:end],
    L"\sigma_{\mu_y} \; \mathrm{[mas \; yr^{-1}]} \; \mathrm{[VERA]}",
    L"\sigma_{\mu_y} \; \mathrm{[mas \; yr^{-1}]} \; \mathrm{[Reid]}",
)
pgfsave(joinpath(PLOTS_DIR, "mu_y$(POSTFIX).pdf"), p)

# Plot a scatter plot for the Local Standard of Rest velocity
println(pad, "    for v_lsr...")
p = scatter(
    e_v_lsr[1:2:end],
    e_v_lsr[2:2:end],
    L"\sigma_{V_{LSR}} \; \mathrm{[km \; s^{-1}]} \; \mathrm{[VERA]}",
    L"\sigma_{V_{LSR}} \; \mathrm{[km \; s^{-1}]} \; \mathrm{[Reid]}",
)
pgfsave(joinpath(PLOTS_DIR, "v_lsr$(POSTFIX).pdf"), p)

println()
