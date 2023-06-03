# This script generates tables from the results for the presentation

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
        { julia --project=. | ./julia.bash } scripts/presentation.jl -i <INPUT_DIR>

        $(YELLOW)OPTIONS:$(RESET)
            $(GREEN)-i <INPUT_DIR>$(RESET)    Input directory"""
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

using CSV
using LaTeXStrings
using Printf

# Define the paths
CURRENT_DIR = @__DIR__
ROOT_DIR = dirname(CURRENT_DIR)
INPUT_DIR = isabspath(INPUT_DIR) ? INPUT_DIR : joinpath(ROOT_DIR, INPUT_DIR)
OUTPUT_DIR = joinpath(ROOT_DIR, "presentation", "tables")

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
    "Near the solar circle (self-consistency check, iter 2)",
    "HMSFRs",
    "HMSFRs (disabled inner optimization)",
]

TABLES = [
    "solar_sc",
    "hmsfrs",
    "hmsfrs_di",
]

N_MAX = [
    6,
    8,
    8,
]

N_RANGE = [
    1:3,
    1:4,
    1:4,
]

BEST_N = [
    [1],
    [3],
    [3, 4],
]

CAPTIONS = [
    raw"Результаты для самосогласованной выборки мазеров вблизи солнечного круга",
    raw"Результаты для выборки HMSFRs",
    raw"Результаты для выборки HMSFRs без внутр. оптимизации",
]

PARAMS_NAMES = [
    L"R_0",
    L"\omega_0",
    L"A",
    L"u_\odot",
    L"v_\odot",
    L"w_\odot",
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

PARAMS_N = length(PARAMS_NAMES)

PARAMS_UNITS = [
    raw"кпк",
    raw"км/с/кпк",
    raw"км/с/кпк",
    raw"км/с",
    raw"км/с",
    raw"км/с",
    raw"км/с",
    raw"км/с",
    raw"км/с",
    raw"км/с/кпк$^2$",
    raw"км/с/кпк$^3$",
    raw"км/с/кпк$^4$",
    raw"км/с/кпк$^5$",
    raw"км/с/кпк$^6$",
    raw"км/с/кпк$^7$",
    raw"км/с/кпк$^8$",
    raw"км/с",
    raw"км/с",
    raw"км/с",
    raw"км/с/кпк",
]

function find_new_digits(param_e_string)
    dot_index = findfirst(".", param_e_string).start
    integer_index = param_e_string[dot_index-1]
    significant_index = findfirst(c -> c != '0', param_e_string[dot_index+1:end])
    return if integer_index == '1'
        2
    elseif integer_index != '0'
        1
    elseif significant_index == nothing
        2
    else
        number = param_e_string[dot_index + significant_index]
        if number == '1'
            significant_index + 2
        else
            significant_index + 1
        end
    end
end

function get_new_format(param_ep, param_em)
    digits = 8
    format = Printf.Format("%.$(digits)f")

    param_ep_string = Printf.format(format, param_ep)
    param_em_string = Printf.format(format, param_em)

    new_digits_ep = find_new_digits(param_ep_string)
    new_digits_em = find_new_digits(param_em_string)

    new_digits = min(new_digits_ep, new_digits_em)

    return Printf.Format("%.$(new_digits)f")
end

for i in 1:length(SAMPLES)
    sample = SAMPLES[i]
    table = TABLES[i]
    n_max = N_MAX[i]
    n_range = N_RANGE[i]
    best_n = BEST_N[i]
    caption = CAPTIONS[i]

    open(joinpath(OUTPUT_DIR, table * ".tex"), "w") do io
        println(
            io,
            """
            \\begin{table}
              \\scriptsize
              \\centering
              \\caption{$(caption)}
              \\tabcolsep=5pt
              \\renewcommand{\\arraystretch}{1.25}
              \\renewcommand\\cellset{\\renewcommand{\\arraystretch}{0.7}}
              \\begin{tabular}{c|rrrr}
                \\noalign{\\hrule height 0.75pt}"""
        )
        println(io, "    \$ n \$ " * join(map(n -> n in best_n ? "& \$ \\mathbf{$(n)} \$ " : "& \$ $(n) \$ ", n_range)) * "\\\\")
        println(io, raw"    \hline\\[-1.2em]")

        all_fit_params_data = ParamsData();

        for n in 1:n_max
            fit_params_data_path = joinpath(INPUT_DIR, sample, "n = $(n)", "fit_params.bin")
            fit_params_data = read_bincode(fit_params_data_path, ParamsData)

            for field in fieldnames(ParamsData)
                value = getfield(fit_params_data, field)[1]
                vec = getfield(all_fit_params_data, field)
                push!(vec, value)
            end
        end

        fit_params_data = nothing

        digits = 3
        format = Printf.Format("%.$(digits)f")

        for i in 1:(8 + maximum(n_range))
            param_name = PARAMS_NAMES[i]
            param_units = PARAMS_UNITS[i]

            params = getfield(all_fit_params_data, 1 + 3 * (i - 1))
            params_ep = getfield(all_fit_params_data, 2 + 3 * (i - 1))
            params_em = getfield(all_fit_params_data, 3 + 3 * (i - 1))

            if i > 9 && i < 17 && i - 8 > n_max
                continue
            end

            if i == 17
                println(io, raw"    \midrule")
            end

            line = "    \\makecell[tc]{ $(param_name) \\\\ \\scriptsize \\textrm{($(param_units))} } "

            for (n, (param, param_ep, param_em)) in enumerate(zip(params, params_ep, params_em))
                if !(n in n_range)
                    continue
                elseif i > 9 + (n - 1) && i < 17
                    line *= "& --- "
                else
                    new_format = get_new_format(param_ep, param_em)

                    param_string = Printf.format(new_format, param)
                    param_ep_string = Printf.format(new_format, param_ep)
                    param_em_string = Printf.format(new_format, param_em)

                    if n in best_n
                        line *= "& \$ \\mathbf{$(param_string)}"
                    else
                        line *= "& \$ $(param_string)"
                    end
                    line *= "^{\\scriptsize +$(param_ep_string)}"
                    line *= "_{\\scriptsize -$(param_em_string)} \$"
                end
            end

            if i == (8 + maximum(n_range))
                line *= "\\\\[1.2em]"
            else
                line *= "\\\\"
            end

            println(io, line)
        end

        all_fit_params_data = nothing

        println(
            io,
            """
                \\noalign{\\hrule height 0.75pt}
              \\end{tabular}
            \\end{table}"""
        )
    end
end

println()
