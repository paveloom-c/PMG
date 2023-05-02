# This script creates plots based on the extra
# information about the blacklisted objects

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
        { julia --project=. | ./julia.bash } scripts/blacklisted.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [--postfix <POSTFIX>]

        $(YELLOW)OPTIONS:$(RESET)
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
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? OUTPUT_DIR : joinpath(ROOT_DIR, OUTPUT_DIR)
BLACKLISTED_OBJECTS_DIR = joinpath(INPUT_DIR, "Blacklisted objects")
INNER_PROFILES_DIR = joinpath(BLACKLISTED_OBJECTS_DIR, "Inner profiles")
COORDS_DATA_PATH = joinpath(BLACKLISTED_OBJECTS_DIR, "coords.dat")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

println(pad, "> Loading the data...")

coords_data = CSV.File(COORDS_DATA_PATH, delim=' ', comment="#")

X = coords_data.X
X_p = coords_data.X_p
X_m = coords_data.X_m
Y = coords_data.Y
Y_p = coords_data.Y_p
Y_m = coords_data.Y_m
X_r = coords_data.X_r
Y_r = coords_data.Y_r

println(pad, "> Plotting the XY distribution...")

"Compute the limits from the collection"
function max_min(c; factor=0.1)
    max_value = maximum(c)
    min_value = minimum(c)
    len = max_value - min_value
    max_value = max_value + factor * len
    min_value = min_value - factor * len
    return max_value, min_value
end

function scatter(
    x,
    y,
    x_r,
    y_r,
    xlabel,
    ylabel;
    x_p=F[],
    x_m=F[],
    y_p=F[],
    y_m=F[],
)
    # Compute the limits
    x_max, x_min = max_min(x)
    y_max, y_min = max_min(y)
    x_r_max, x_r_min = max_min(x_r)
    y_r_max, y_r_min = max_min(y_r)
    # Create a plot
    p = @pgf Axis(
        {
            xlabel = xlabel,
            ylabel = ylabel,
            xmax = max(x_max, x_r_max),
            xmin = min(x_min, x_r_min),
            ymax = max(y_max, y_r_max),
            ymin = min(y_min, y_r_min),
            height = 200,
            width = 200,
            grid = "both",
            minor_tick_num = 4,
            minor_grid_style = {opacity = 0.25},
            major_grid_style = {opacity = 0.5},
            tick_label_style = {font = "\\small"},
            tick_style = {line_width = 0.4, color = "black"},
            axis_equal = true,
            axis_on_top = true,
            axis_line_style = {line_width = 1},
            "axis_lines*" = "left",
            legend_image_post_style = {mark_size = 2, line_width = 0.4},
            legend_pos = "outer north east",
            legend_style = {line_width = 1},
            mark_size = 0.5,
            line_width = 0.15,
        },
        Plot(
            {
                only_marks,
                color = colors[2],
            },
            Table(
                x=x,
                y=y,
            ),
        ),
        LegendEntry("observed"),
        Plot(
            {
                only_marks,
                color = colors[1],
            },
            Table(
                x=x_r,
                y=y_r,
            ),
        ),
        LegendEntry("reduced"),
    )
    # Add the error lines if additional data sets are specified
    if isempty(x_p) && isempty(x_m) && isempty(y_p) && isempty(y_m)
        for (x, y, x_r, y_r) in zip(x, y, x_r, y_r)
            push!(p, @pgf Plot(
                {
                    no_marks,
                    opacity = 0.25,
                    line_width = 0.1,
                },
                Coordinates([(x, y), (x_r, y_r)])
            ))
        end
    else
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
    p = scatter(
        X,
        Y,
        X_r,
        Y_r,
        L"X \; \mathrm{[kpc]}",
        L"Y \; \mathrm{[kpc]}",
    )
    pgfsave(joinpath(BLACKLISTED_OBJECTS_DIR, "XY$(POSTFIX).pdf"), p)
end)

push!(tasks, @spawn begin
    lock(print_lock) do
        println(pad, pad, "with errors...")
    end
    p = scatter(
        X,
        Y,
        X_r,
        Y_r,
        L"X \; \mathrm{[kpc]}",
        L"Y \; \mathrm{[kpc]}",
        x_p=X_p,
        x_m=X_m,
        y_p=Y_p,
        y_m=Y_m,
    )
    pgfsave(joinpath(BLACKLISTED_OBJECTS_DIR, "XY (errors)$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

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
for profile_path in readdir(INNER_PROFILES_DIR, join=true)
    if !endswith(profile_path, ".dat")
        continue
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
            L"\varpi",
            L"\sum",
        )
        pgfsave(
            joinpath(
                INNER_PROFILES_DIR,
                replace(profile_name, ".dat" => ".pdf"),
            ),
            p,
        )
    end)
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
