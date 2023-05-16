# This script creates plots based on the extra
# information about the outliers

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
PLOT_TEST = false
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
        { julia --project=. | ./julia.bash } scripts/outliers.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [--postfix <POSTFIX>]
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

using Base.Threads
using CSV
using ColorSchemes
using LaTeXStrings
using PGFPlotsX

# Choose a color scheme
colors = ColorSchemes.tol_bright[2:end]

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? joinpath(INPUT_DIR, "n = $(N)") : joinpath(ROOT_DIR, INPUT_DIR, "n = $(N)")
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? joinpath(OUTPUT_DIR, "n = $(N)") : joinpath(ROOT_DIR, OUTPUT_DIR, "n = $(N)")
INNER_PROFILES_DIR = joinpath(OUTPUT_DIR, "Inner profiles")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

println(pad, "> Plotting the inner profiles...")

function plot(x, y, xlabel, ylabel)
    # Prepare a table
    table = @pgf Table(
        x=x,
        y=y,
    )
    y_min = minimum(y)
    y_max = min(maximum(y), 1e3)
    # Create a plot
    return @pgf Axis(
        {
            xlabel = xlabel,
            ylabel = ylabel,
            "restrict_y_to_domain"="$(y_min):$(y_max)",
            height = 200,
            width = 200,
            grid = "both",
            minor_grid_style = {opacity = 0.25},
            major_grid_style = {opacity = 0.5},
            tick_label_style = {font = "\\small"},
            tick_style = {line_width = 0.4, color = "black"},
            axis_line_style = {line_width = 1},
            "axis_lines*" = "left",
            mark_size = 0.5,
            line_width = 0.35,
        },
        Plot(
            {
                no_marks,
                color = colors[1],
            },
            table,
        ),
    )
end

print_lock = ReentrantLock()
tasks = Task[]

# Create a plot for each profile
task_count = 0
for profile_path in readdir(INNER_PROFILES_DIR, join=true)
    if !endswith(profile_path, ".dat")
        continue
    end

    # Pause for this many seconds between the batches
    if task_count > 15
        global task_count = 0
        sleep(5)
    end

    profile_data = CSV.File(profile_path, delim=' ', comment="#")
    profile_name = basename(profile_path)
    number = replace(profile_name, ".dat" => "")

    push!(tasks, @spawn begin
        lock(print_lock) do
            println(pad, pad, "for object #$(number)...")
        end
        p = plot(
            profile_data.par_r,
            profile_data.sum,
            L"\varpi_{0,j}",
            L"\sum_m (|\delta_m| / \sigma_m)^2",
        )
        pgfsave(
            joinpath(
                INNER_PROFILES_DIR,
                replace(profile_name, ".dat" => ".pdf"),
            ),
            p,
        )
    end)

    task_count += 1
end

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

# Mark data for garbage collection
coords_data = nothing

println()
