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

# Define default values for the arguments
LEGEND_SHOW_SOURCES = false
INPUT_FILE = ""
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
    # Input file
    if ARGS[i] == "-i"
        try
            global INPUT_FILE = parse_string(i+1)
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
        { julia --project=. | ./julia.bash } scripts/compare.jl -i <INPUT_FILE> -o <OUTPUT_DIR> [--postfix <POSTFIX>]

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-i <INPUT_FILE>$(RESET)        Input file
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files"""
    )
    exit(1)
end

# Make sure the required arguments are passed
if isempty(INPUT_FILE)
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
using CSV
using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Add support for Russian
push!(PGFPlotsX.CUSTOM_PREAMBLE, """
\\usepackage{fontspec}
\\defaultfontfeatures{Ligatures={TeX}}
\\setmainfont{cmun}[
  Extension=.otf,
  UprightFont=*rm,
  ItalicFont=*ti,
  BoldFont=*bx,
  BoldItalicFont=*bi,
]
\\setsansfont{cmun}[
  Extension=.otf,
  UprightFont=*ss,
  ItalicFont=*si,
  BoldFont=*sx,
  BoldItalicFont=*so,
]
\\setmonofont{cmun}[
  Extension=.otf,
  UprightFont=*btl,
  ItalicFont=*bto,
  BoldFont=*tb,
  BoldItalicFont=*tx,
]
\\usepackage[main=russian,english]{babel}""")

# Choose a color scheme
colors = ColorSchemes.tol_bright[2:end]

# Define the markers set
marks = repeat(["*"], 5)

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_FILE = isabspath(INPUT_FILE) ? INPUT_FILE : joinpath(ROOT_DIR, INPUT_FILE)
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? OUTPUT_DIR : joinpath(ROOT_DIR, OUTPUT_DIR)

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the paths to the data files
println(pad, "> Loading the data...")

# Read the data
data = CSV.File(INPUT_FILE, delim=' ', comment="#")
μ_x_e = data.mu_x_e
μ_y_e = data.mu_y_e
v_lsr_e = data.v_lsr_e

println(pad, "> Plotting the comparison charts...")

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
            axis_on_top = true,
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
                only_marks,
            },
            Coordinates(x, y),
        ),
    )
    return p
end

print_lock = ReentrantLock()
tasks = Task[]

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for mu_x...")
    end
    p = scatter(
        μ_x_e[1:2:end],
        μ_x_e[2:2:end],
        L"\sigma_{\mu_x} \; \mathrm{[мсд/г]} \; \mathrm{[VERA]}",
        L"\sigma_{\mu_x} \; \mathrm{[мсд/г]} \; \mathrm{[Reid]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "mu_x$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for mu_y...")
    end
    p = scatter(
        μ_y_e[1:2:end],
        μ_y_e[2:2:end],
        L"\sigma_{\mu_y} \; \mathrm{[мсд/г]} \; \mathrm{[VERA]}",
        L"\sigma_{\mu_y} \; \mathrm{[мсд/г]} \; \mathrm{[Reid]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "mu_y$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "for v_lsr...")
    end
    p = scatter(
        v_lsr_e[1:2:end],
        v_lsr_e[2:2:end],
        L"\sigma_{V_{LSR}} \; \mathrm{[км/с]} \; \mathrm{[VERA]}",
        L"\sigma_{V_{LSR}} \; \mathrm{[км/с]} \; \mathrm{[Reid]}",
    )
    pgfsave(joinpath(OUTPUT_DIR, "v_lsr$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

println()
