# This script plots the dependencies of the costs (L_1)
# and errors in azimuthal velocity on the degree of the
# polynomial of the rotation curve

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
PLOT_TEST = false
INPUT_DIR = ""
OUTPUT_DIR = ""
POSTFIX = ""

# Parse the options
for i in eachindex(ARGS)
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
        { julia --project=. | ./julia.bash } scripts/n.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [--postfix <POSTFIX>]

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-i <INPUT_DIR>$(RESET)         Input directory
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

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? OUTPUT_DIR : joinpath(ROOT_DIR, OUTPUT_DIR)
L_1_DATA_PATH = joinpath(INPUT_DIR, "L_1.dat")
SIGMA_THETA_DATA_PATH = joinpath(INPUT_DIR, "sigma_theta.dat")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

println(pad, "> Loading the data...")

l_1_data = CSV.File(L_1_DATA_PATH, delim=' ', comment="#")
sigma_theta_data = CSV.File(SIGMA_THETA_DATA_PATH, delim=' ', comment="#")

println(pad, "> Plotting the dependencies of...")

"Create a scatter plot"
function scatter(
    x,
    y,
    xlabel,
    ylabel,
)
    # Prepare a table
    table = @pgf Table(
        x=x,
        y=y,
    )
    # Create a plot
    return @pgf Axis(
        {
            xlabel = xlabel,
            ylabel = ylabel,
            height = 200,
            width = 200,
            grid = "both",
            xtick_distance = 1,
            minor_grid_style = {opacity = 0.25},
            major_grid_style = {opacity = 0.5},
            tick_label_style = {font = "\\small"},
            tick_style = {line_width = 0.4, color = "black"},
            axis_line_style = {line_width = 1},
            "axis_lines*" = "left",
            mark_size = 1,
        },
        Plot(
            {
                only_marks,
                color = colors[1],
            },
            table,
        ),
    )
end

print_lock = ReentrantLock()
tasks = Task[]

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "`L_1` on `n`...")
    end
    p = scatter(
        l_1_data.n,
        l_1_data.L_1,
        L"n",
        L"\mathcal{L}^{(1)}",
    )
    pgfsave(
        joinpath(
            OUTPUT_DIR,
            "L_1.pdf",
        ),
        p,
    )
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "`sigma_theta` on `n`...")
    end
    p = scatter(
        sigma_theta_data.n,
        sigma_theta_data.sigma_theta,
        L"n",
        L"\sigma_\theta, \, \mathrm{км/с}",
    )
    pgfsave(
        joinpath(
            OUTPUT_DIR,
            "sigma_theta.pdf",
        ),
        p,
    )
end)

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

# Mark data for garbage collection
l_1_data = nothing
sigma_theta_data = nothing

println()
