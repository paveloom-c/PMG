# This script plots the profiles

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
        { julia --project=. | ./julia.bash } scripts/profiles.jl -i <INPUT_DIR> -o <OUTPUT_DIR> [--postfix <POSTFIX>]

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
FIT_PARAMS_DATA_PATH = joinpath(INPUT_DIR, "fit_params.bin")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

if isfile(FIT_PARAMS_DATA_PATH)
    println(pad, "> Loading the data...")

    include(joinpath(CURRENT_DIR, "data_types.jl"))

    ParamsData = Types.Params{F}
    ProfileData = Types.Profile{F}

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

    # Read the parameters data
    fit_params_data = read_bincode(FIT_PARAMS_DATA_PATH, ParamsData)

    println(pad, "> Plotting the profiles...")

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
        ylabel,
        fit_param,
        fit_param_p,
        fit_param_m,
        with_errors=false,
    )
        # Compute the limits
        x_max, x_min = max_min(x)
        y_max, y_min = max_min(y; factor=0.5)
        # Compute the X tick distance
        x_diff = (x_max - x_min)
        xtick_distance = if x_diff < 1
            0.1
        elseif x_diff < 5
            0.5
        else
            1
        end
        # Prepare a table
        table = @pgf Table(
            x=x,
            y=y,
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
                "restrict_y_to_domain"="$(y_min):$(y_max)",
                height = 200,
                width = 200,
                grid = "both",
                xtick_distance = xtick_distance,
                minor_x_tick_num = 4,
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
            Plot(
                {
                    dashed,
                    no_marks,
                    color = colors[2]
                },
                Coordinates([
                    (fit_param, y_min + 0.04 * (y_max - y_min)),
                    (fit_param, y_max - 0.015 * (y_max - y_min)),
                ]),
            ),
        )
        if with_errors
            push!(p, @pgf Plot(
                {
                    dashed,
                    no_marks,
                    color = colors[3]
                },
                Coordinates([
                    (fit_param_p, y_min + 0.04 * (y_max - y_min)),
                    (fit_param_p, y_max - 0.015 * (y_max - y_min)),
                ]),
            ))
            push!(p, @pgf Plot(
                {
                    dashed,
                    no_marks,
                    color = colors[3]
                },
                Coordinates([
                    (fit_param_m, y_min + 0.04 * (y_max - y_min)),
                    (fit_param_m, y_max - 0.015 * (y_max - y_min)),
                ]),
            ))
        end
        return p
    end

    print_lock = ReentrantLock()
    tasks = Task[]

    PARAMS_NAMES = [
        "R_0",
        "omega_0",
        "A",
        "U_sun",
        "V_sun",
        "W_sun",
        "sigma_R",
        "sigma_theta",
        "sigma_Z",
        "theta_2",
        "theta_3",
        "theta_4",
        "theta_5",
        "theta_6",
        "theta_7",
        "theta_8",
        "omega_sun",
    ]

    PARAMS_N = length(PARAMS_NAMES) - 1

    PARAMS_LATEX_NAMES = [
        L"R_0",
        L"\omega_0",
        L"A",
        L"U_\odot",
        L"V_\odot",
        L"W_\odot",
        L"\sigma_R",
        L"\sigma_\theta",
        L"\sigma_Z",
        L"\theta_2",
        L"\theta_3",
        L"\theta_4",
        L"\theta_5",
        L"\theta_6",
        L"\theta_7",
        L"\theta_8",
        L"\omega_\odot",
    ]

    PARAMS_LATEX_UNITS = [
        L"\; \mathrm{[kpc]}",
        L"\; \mathrm{[km/s/kpc]}",
        L"\; \mathrm{[km/s/kpc]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s]}",
        L"\; \mathrm{[km/s/kpc^2]}",
        L"\; \mathrm{[km/s/kpc^3]}",
        L"\; \mathrm{[km/s/kpc^4]}",
        L"\; \mathrm{[km/s/kpc^5]}",
        L"\; \mathrm{[km/s/kpc^6]}",
        L"\; \mathrm{[km/s/kpc^7]}",
        L"\; \mathrm{[km/s/kpc^8]}",
        L"\; \mathrm{[km/s/kpc]}",
    ]

    fit_params_vec = map(i -> getfield(fit_params_data, i)[1], 1:fieldcount(ParamsData))
    fit_params = push!(fit_params_vec[1:3:PARAMS_N*3], fit_params_data.ω_sun[1])
    fit_params_ep = push!(fit_params_vec[2:3:PARAMS_N*3], fit_params_data.ω_sun_ep[1])
    fit_params_em = push!(fit_params_vec[3:3:PARAMS_N*3], fit_params_data.ω_sun_em[1])

    prefixes = ["conditional", "frozen"]

    # Create a plot for each profile
    for prefix in prefixes
        for i in 1:length(PARAMS_NAMES)
            name = PARAMS_NAMES[i]
            latex_name = PARAMS_LATEX_NAMES[i]
            latex_unit = PARAMS_LATEX_UNITS[i]

            profile_path = joinpath(INPUT_DIR, "$(prefix)_profile_$(name).bin")
            if !isfile(profile_path) continue end

            fit_param = fit_params[i]
            fit_param_ep = fit_params_ep[i]
            fit_param_em = fit_params_em[i]

            fit_param_p = fit_param + fit_param_ep
            fit_param_m = fit_param - fit_param_ep

            profile_data = read_bincode(profile_path, ProfileData)

            param = profile_data.param
            cost = profile_data.cost

            ylabel = prefix == "conditional" ? L"L_c" : L"L_f"

            push!(tasks, @spawn begin
                lock(print_lock) do
                    println(pad, pad, "$(prefix) for $(name)...")
                end
                with_errors = prefix == "conditional"
                p = scatter(
                    param,
                    cost,
                    latexstring(latex_name, latex_unit),
                    latexstring(ylabel, L"(", latex_name, L")"),
                    fit_param,
                    fit_param_p,
                    fit_param_m,
                    with_errors,
                )
                pgfsave(
                    joinpath(
                        OUTPUT_DIR,
                        "$(titlecase(prefix)) profile of $(name)$(POSTFIX).pdf",
                    ),
                    p,
                )
            end)

            profile_data = nothing
        end
    end

    for task in tasks
        try
            wait(task)
        catch err
            showerror(stdout, err.task.exception)
        end
    end

    # Mark data for garbage collection
    fit_params_data = nothing
end

println()
