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
N = nothing
LEGEND_SHOW_SOURCES = false
INPUT_DIR = ""
OUTPUT_DIR = ""
POSTFIX = ""
NO_DISTANCE_ERRORS = false

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
    # Don't plot distance errors
    if ARGS[i] == "--no-distance-errors"
        global NO_DISTANCE_ERRORS = true
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
            $(GREEN)-n$(RESET)                     Degree of the polynomial of the rotation curve
            $(GREEN)-s$(RESET)                     Show sources on the legend instead of types
            $(GREEN)-i <INPUT_DIR>$(RESET)         Input directory
            $(GREEN)-o <OUTPUT_DIR>$(RESET)        Output directory
            $(GREEN)--postfix <POSTFIX>$(RESET)    A postfix for the names of output files
            $(GREEN)--no-distance-errors$(RESET)   Don't plot distance errors"""
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
colors = ColorSchemes.tol_bright

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? joinpath(INPUT_DIR, "n = $(N)") : joinpath(ROOT_DIR, INPUT_DIR, "n = $(N)")
OUTPUT_DIR = isabspath(OUTPUT_DIR) ? joinpath(OUTPUT_DIR, "n = $(N)") : joinpath(ROOT_DIR, OUTPUT_DIR, "n = $(N)")
OBJECTS_DATA_PATH = joinpath(INPUT_DIR, "fit_objects.bin")
FIT_ROTCURVE_DATA_PATH = joinpath(INPUT_DIR, "fit_rotcurve.bin")
FIT_PARAMS_DATA_PATH = joinpath(INPUT_DIR, "fit_params.bin")

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
source = objects_data.source[I]
outliers = objects_data.outlier[I]
R = objects_data.R[I]
R_p = objects_data.R_p[I]
R_m = objects_data.R_m[I]
Θ = objects_data.Θ[I]
Θ_p = objects_data.Θ_p[I]
Θ_m = objects_data.Θ_m[I]
θ_evel = objects_data.θ_evel[I]
θ_evel_corrected = objects_data.θ_evel_corrected[I]

# Unpack the fit data
R_fit = fit_rotcurve_data.R
Θ_fit = fit_rotcurve_data.Θ
σ_fit = fit_rotcurve_data.σ
R_0 = fit_params_data.R_0[1]
θ_sun = fit_params_data.θ_sun[1]
σ_θ = fit_params_data.σ_θ[1]

# Prepare labels
markers = ["a", "b", "c", "d", "e"]
outliers_markers = ["f", "g", "h", "i", "j"]
dictionary = Dict([(k, markers[i]) for (i, k) in enumerate(keys)])
outliers_dictionary = Dict([(k, outliers_markers[i]) for (i, k) in enumerate(keys)])
label = [o ? outliers_dictionary[k] : dictionary[k] for (k, o) in zip(group, outliers)]

# Compute a mask for the non-from-Reid objects
NR = findall(s -> s != "Reid", source)

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
    e_fit,
    xlabel,
    ylabel;
    x_p=F[],
    x_m=F[],
    y_p=F[],
    y_m=F[],
    evel=F[],
    evel_corrected=F[],
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
            "restrict_y_to_domain*"="$(y_min):$(y_max)",
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
            axis_line_style = {line_width = 1},
            axis_on_top = true,
            "axis_lines*" = "left",
            legend_image_post_style = {mark_size = 2, line_width = 0.4},
            legend_pos = "outer north east",
            legend_style = {line_width = 1, label="below: {\$ n = $(N) \$}"},
            mark_size = 0.5,
            line_width = 0.15,
            "scatter/classes" = {
                a = {mark = "x", color = colors[2]},
                b = {mark = "+", color = colors[3]},
                c = {mark = "asterisk", color = colors[4]},
                d = {mark = "star", color = colors[5]},
                e = {mark = "10-pointed star", color = colors[6]},
                f = {mark = "o", color = colors[2], mark_size = 0.85 },
                g = {mark = "o", color = colors[3], mark_size = 0.85 },
                h = {mark = "o", color = colors[4], mark_size = 0.85 },
                i = {mark = "o", color = colors[5], mark_size = 0.85 },
                j = {mark = "o", color = colors[6], mark_size = 0.85 },
            },
        },
    )
    # Add second pair of bars to the non-from-Reid objects
    if !isempty(evel_corrected)
        push!(p, @pgf Plot(
            {
                scatter,
                only_marks,
                mark="none",
                color = colors[1],
                "error bars/y dir=both",
                "error bars/y explicit",
                "error bars/error bar style" = {line_width = 0.1, opacity = 0.75},
                "error bars/error mark options" = {
                    rotate = 90,
                    mark_size = 0.5,
                    line_width = 0.1,
                    opacity = 0.75,
                },
            },
            Table(
                {
                    y_error = "evel_corrected",
                },
                x=x[NR],
                y=y[NR],
                evel_corrected=evel_corrected[NR],
            ),
        ))
    end
    push!(p, @pgf [
        Plot(
            {
                scatter,
                only_marks,
                scatter_src = "explicit symbolic",
                "error bars/y dir=both",
                "error bars/y explicit",
                "error bars/error bar style" = {line_width = 0.1, opacity = 0.25},
                "error bars/error mark options" = {
                    rotate = 90,
                    mark_size = 0.5,
                    line_width = 0.1,
                    opacity = 0.25,
                },
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
        [raw"\node[font=\fontsize{1}{0}\selectfont] at ", Coordinate(R_0, θ_sun), raw"{$\odot$};"],
        Legend(keys),
    ])
    # Add the error lines if additional data sets are specified
    if !NO_DISTANCE_ERRORS && !isempty(x_p) && !isempty(x_m) && !isempty(y_p) && !isempty(y_m)
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
    # Add confidence intervals if they were computed
    if !iszero(e_fit)
        push!(p, @pgf [
            Plot(
                {
                    color = colors[3],
                    mark = "none"
                },
                Table(
                    x=x_fit,
                    y=y_fit .+ e_fit,
                ),
            ),
            Plot(
                {
                    color = colors[3],
                    mark = "none"
                },
                Table(
                    x=x_fit,
                    y=y_fit .- e_fit,
                ),
            ),
        ])
    end
    # Add a bar for the azimuthal component of the velocity ellipsoid
    push!(p, @pgf Plot(
        {
            scatter,
            point_meta="explicit symbolic",
            nodes_near_coords,
            nodes_near_coords_style={font = raw"\scriptsize", anchor="west"},
            "error bars/y dir=both",
            "error bars/y explicit",
            "error bars/error bar style" = {line_width = 0.1, opacity = 0.25},
            "error bars/error mark options" = {
                rotate = 90,
                mark_size = 0.5,
                line_width = 0.1,
                opacity = 0.25,
            },
        },
        Table(
            {
                y_error="yerror",
                meta="meta",
            },
            x=[x_min + (x_max - x_min) * 0.1],
            y=[300],
            yerror=[σ_θ],
            meta=[L"\pm\sigma_\theta"];
        ),
    ))
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
        σ_fit,
        L"R, \, \mathrm{кпк}",
        L"\theta, \, \mathrm{км/с}",
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
        σ_fit,
        L"R, \, \mathrm{кпк}",
        L"\theta, \, \mathrm{км/с}",
        x_p=R_p,
        x_m=R_m,
        y_p=Θ_p,
        y_m=Θ_m,
        evel=θ_evel,
        evel_corrected=θ_evel_corrected,
    )
    pgfsave(joinpath(OUTPUT_DIR, "Fitted rotation curve (errors)$(POSTFIX).pdf"), p)
end)

for task in tasks
    try
        wait(task)
    catch err
        showerror(stdout, err.task.exception)
    end
end

# Mark data for garbage collection
objects_data = nothing
fit_rotcurve_data = nothing
fit_params_data = nothing

println()
