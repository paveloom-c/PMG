# This script generates tables from the results

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
INPUT_DIR = ""

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
end

# Prepare color codes
RESET = "\e[0m"
GREEN = "\e[32m"
YELLOW = "\e[33m"

# Show help if requested
if "--help" in ARGS
    println("""
        $(YELLOW)USAGE:$(RESET)
        { julia --project=. | ./julia.bash } diploma/tables.jl -i <INPUT_DIR>

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-i <INPUT_DIR>$(RESET)         Input directory"""
    )
    exit(1)
end

# Make sure the required arguments are passed
if isempty(INPUT_DIR)
    println("An input file is required.")
    exit(1)
end

"Padding in the output"
pad = " "^4

"Floating point type used across the script"
F = Float64

"Integer type used across the script"
I = UInt64

println('\n', pad, "> Loading the packages...")

using LaTeXStrings
using Printf

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
OUTPUT_DIR = joinpath(ROOT_DIR, "diploma", "tables")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

println(pad, "> Loading the data...")

include(joinpath(CURRENT_DIR, "data_types.jl"))

ParamsData = Types.Params{F}

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

SAMPLES = [
    "Near the solar circle",
    "HMSFRs",
]

TABLES = [
    "solar",
    "hmsfrs",
]

N_MAX = [
    6,
    8,
]

BEST_N = [
    1,
    3,
]

CAPTIONS = [
    "Результаты для околосолнечной выборки (жирным выделен оптимальный порядок)",
    "Результаты для выборки HMSFRs (жирным выделен оптимальный порядок)",
]

PARAMS_N = 16

PARAMS_NAMES = [
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
    L"\theta_0",
    L"\theta_1",
    L"\theta_\odot",
    L"\omega_\odot",
]

for i in 1:length(SAMPLES)
    sample = SAMPLES[i]
    table = TABLES[i]
    n_max = N_MAX[i]
    best_n = BEST_N[i]
    caption = CAPTIONS[i]

    open(joinpath(OUTPUT_DIR, table * ".tex"), "w") do io
        println(
            io,
            raw"""
            \begin{table}[H]
              \centering
              \aboverulesep=0ex
              \belowrulesep=0ex
              \renewcommand{\arraystretch}{1.25}
              \renewcommand\cellset{\renewcommand{\arraystretch}{0.7}}
              \begin{tabular}{c|rrrrrrrr}
                \toprule"""
        )
        println(io, "    \$ n \$ " * join(map(n -> n == best_n ? "& \\textbf{$(n)} " : "& $(n) ", 1:n_max)) * "\\\\")
        println(io, raw"    \midrule")

        all_fit_params_data = ParamsData();

        for n in 1:n_max
            fit_params_data_path = joinpath(INPUT_DIR, sample, "n = $(n)", "fit_params.bin")
            fit_params_data = read_bincode(fit_params_data_path, ParamsData)

            for field in fieldnames(ParamsData)
                value = getfield(fit_params_data, field)[1]
                vec = getfield(all_fit_params_data, field)
                push!(vec, value)
            end

            fit_params_data = nothing
        end

        digits = 4

        for i in 1:(PARAMS_N - (8 - n_max))
            param_name = PARAMS_NAMES[i]

            params = getfield(all_fit_params_data, 1 + 3 * (i - 1))
            params_ep = getfield(all_fit_params_data, 2 + 3 * (i - 1))
            params_em = getfield(all_fit_params_data, 3 + 3 * (i - 1))

            line = "    $(param_name) "

            for (n, (param, param_ep, param_em)) in enumerate(zip(params, params_ep, params_em))
                if i > 9 + (n - 1)
                    line *= "& --- "
                else
                    format = Printf.Format("%.$(digits)f")
                    param_string = Printf.format(format, param)
                    param_ep_string = Printf.format(format, param_ep)
                    param_em_string = Printf.format(format, param_em)
                    if n == best_n
                        line *= "& \\makecell[tr]{ \$ \\mathbf{$(param_string)} \$ \\\\"
                    else
                        line *= "& \\makecell[tr]{ \$ $(param_string) \$ \\\\"
                    end
                    line *= " \\scriptsize \$ +$(param_ep_string) \$ \\\\"
                    line *= " \\scriptsize \$ -$(param_em_string) \$ } "
                end
            end

            line *= "\\\\"

            println(io, line)
        end

        println(io, raw"    \midrule")

        for i in 17:19
            param_name = PARAMS_NAMES[i]

            params = getfield(all_fit_params_data, PARAMS_N * 3 + i - 16)

            line = "    $(param_name) "

            for (n, param) in enumerate(params)
                format = Printf.Format("%.$(digits)f")
                param_string = Printf.format(format, param)
                if n == best_n
                    line *= "& \$ \\mathbf{$(param_string)} \$ "
                else
                    line *= "& \$ $(param_string) \$ "
                end
            end

            line *= "\\\\"

            println(io, line)
        end

        println(io, raw"    \midrule")

        let i = 20
            param_name = PARAMS_NAMES[end]

            params = getfield(all_fit_params_data, fieldcount(ParamsData) - 2)
            params_ep = getfield(all_fit_params_data, fieldcount(ParamsData) - 1)
            params_em = getfield(all_fit_params_data, fieldcount(ParamsData))

            line = "    $(param_name) "

            for (n, (param, param_ep, param_em)) in enumerate(zip(params, params_ep, params_em))
                format = Printf.Format("%.$(digits)f")
                param_string = Printf.format(format, param)
                param_ep_string = Printf.format(format, param_ep)
                param_em_string = Printf.format(format, param_em)
                if n == best_n
                    line *= "& \\makecell[tr]{ \$ \\mathbf{$(param_string)} \$ \\\\"
                else
                    line *= "& \\makecell[tr]{ \$ $(param_string) \$ \\\\"
                end
                line *= " \\scriptsize \$ +$(param_ep_string) \$ \\\\"
                line *= " \\scriptsize \$ -$(param_em_string) \$ } "
            end

            line *= "\\\\"

            println(io, line)
        end

        println(
            io,
            """
                \\bottomrule
              \\end{tabular}
              \\caption{$(caption)}
              \\label{table:$(table)}
            \\end{table}"""
        )
    end
end

println()
