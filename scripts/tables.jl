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
    "Near the solar circle (disabled inner optimization)",
    "HMSFRs (disabled inner optimization)",
    "HMSFRs (optimal sample, disabled inner optimization and outliers checks)"
]

TABLES = [
    "solar",
    "hmsfrs",
    "solar_di",
    "hmsfrs_di",
    "hmsfrs_optimal"
]

N_MAX = [
    6,
    8,
    6,
    8,
    8,
]

BEST_N = [
    [1],
    [3],
    [1],
    [3, 4],
    [4],
]

CAPTIONS = [
    raw"Результаты для околосолнечной выборки (жирным выделен оптимальный порядок).",
    raw"Результаты для выборки HMSFRs (жирным выделен оптимальный порядок).",
    raw"Результаты для околосолнечной выборки с отключенной внутренней оптимизацией (жирным выделен оптимальный порядок).",
    raw"Результаты для выборки HMSFRs с отключенной внутренней оптимизацией (жирным выделены оптимальные порядки).",
    raw"Результаты для оптимальной выборки HMSFRs с отключенными внутренней оптимизацией и проверкой на выбросы (жирным выделен оптимальный порядок).",
]

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
              \tabcolsep=5pt
              \renewcommand{\arraystretch}{1.25}
              \renewcommand\cellset{\renewcommand{\arraystretch}{0.7}}
              \begin{tabular}{c|rrrrrrrr}
                \toprule"""
        )
        println(io, "    \$ n \$ " * join(map(n -> n in best_n ? "& \\textbf{$(n)} " : "& $(n) ", 1:n_max)) * "\\\\")
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
        format = Printf.Format("%.$(digits)f")

        for i in 1:PARAMS_N
            param_name = PARAMS_NAMES[i]
            param_units = PARAMS_UNITS[i]

            params = getfield(all_fit_params_data, 1 + 3 * (i - 1))
            params_ep = getfield(all_fit_params_data, 2 + 3 * (i - 1))
            params_em = getfield(all_fit_params_data, 3 + 3 * (i - 1))

            if i == 17
                println(io, raw"    \midrule")
            end

            line = "    \\makecell[tc]{ $(param_name) \\\\ \\scriptsize ($(param_units)) } "

            for (n, (param, param_ep, param_em)) in enumerate(zip(params, params_ep, params_em))
                if i > 9 + (n - 1)
                    line *= "& --- "
                else
                    param_string = Printf.format(format, param)
                    param_ep_string = Printf.format(format, param_ep)
                    param_em_string = Printf.format(format, param_em)
                    if n in best_n
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

        all_fit_params_data = nothing

        delta_varpi_data_path = joinpath(INPUT_DIR, sample, "Delta_varpi.dat")

        if isfile(delta_varpi_data_path)
            delta_varpi_data = CSV.File(
                delta_varpi_data_path,
                delim=' ',
                comment="#",
            )

            println(io, raw"    \midrule")

            begin
                line = raw"    \makecell[tc]{ $ \Delta\varpi $ \\ \scriptsize (мсд) } "

                ns = delta_varpi_data.n
                params = delta_varpi_data.x_mean
                params_e = delta_varpi_data.sigma_x_mean

                for (n, param, param_e) in zip(ns, params, params_e)
                    param_string = Printf.format(format, param)
                    param_e_string = Printf.format(format, param_e)
                    if n in best_n
                        line *= "& \\makecell[tr]{ \$ \\mathbf{$(param_string)} \$ \\\\"
                    else
                        line *= "& \\makecell[tr]{ \$ $(param_string) \$ \\\\"
                    end
                    line *= " \\scriptsize \$ \\pm$(param_e_string) \$ } "
                end

                line *= "\\\\"

                println(io, line)
            end

            begin
                line = raw"    $ \sigma $ "

                ns = delta_varpi_data.n
                params = delta_varpi_data.sigma
                params_e = delta_varpi_data.sigma_sigma

                for (n, param, param_e) in zip(ns, params, params_e)
                    param_string = Printf.format(format, param)
                    param_e_string = Printf.format(format, param_e)
                    if n in best_n
                        line *= "& \\makecell[tr]{ \$ \\mathbf{$(param_string)} \$ \\\\"
                    else
                        line *= "& \\makecell[tr]{ \$ $(param_string) \$ \\\\"
                    end
                    line *= " \\scriptsize \$ \\pm$(param_e_string) \$ } "
                end

                line *= "\\\\"

                println(io, line)
            end

            begin
                line = raw"    $ \sigma^{-1} $ "

                ns = delta_varpi_data.n
                params = delta_varpi_data.sigma_r
                params_e = delta_varpi_data.sigma_sigma_r

                for (n, param, param_e) in zip(ns, params, params_e)
                    param_string = Printf.format(format, param)
                    param_e_string = Printf.format(format, param_e)
                    if n in best_n
                        line *= "& \\makecell[tr]{ \$ \\mathbf{$(param_string)} \$ \\\\"
                    else
                        line *= "& \\makecell[tr]{ \$ $(param_string) \$ \\\\"
                    end
                    line *= " \\scriptsize \$ \\pm$(param_e_string) \$ } "
                end

                line *= "\\\\"

                println(io, line)
            end

            begin
                line = raw"    \makecell[tc]{ $ \sigma' $ \\ \scriptsize (мсд) } "

                ns = delta_varpi_data.n
                params = delta_varpi_data.sigma_stroke

                for (n, param, param_e) in zip(ns, params, params_e)
                    param_string = Printf.format(format, param)
                    param_e_string = Printf.format(format, param_e)
                    if n in best_n
                        line *= "& \$ \\mathbf{$(param_string)} \$ "
                    else
                        line *= "& \$ $(param_string) \$ "
                    end
                end

                line *= "\\\\"

                println(io, line)
            end

            delta_varpi_data = nothing
        end

        n_data = CSV.File(
            joinpath(INPUT_DIR, sample, "n.dat"),
            delim=' ',
            comment="#",
        )

        if length(unique(n_data.n)) > 1
            println(io, raw"    \midrule")
            println(io, "    & \\multicolumn{$(n_max)}{c}{\$ N = $(n_data.n[1]) \$ \\hfill \$ N_{L' = 3} = $(n_data.n[2]) \$ \\hfill \$ N_{L' = 1} = $(n_data.n[3]) \$} \\\\")
        end

        n_data = nothing

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

input_data = CSV.File(
    joinpath(ROOT_DIR, "data", "input", "all.dat"),
    delim=' ',
    comment="#",
)

open(joinpath(OUTPUT_DIR, "catalog.tex"), "w") do io
    println(
        io,
        raw"""
        \begin{sidewaystable}
          \centering
          \aboverulesep=1ex
          \belowrulesep=1ex
          \renewcommand{\arraystretch}{1.25}
          \begin{tabular}{lccccccccccccc}
            \toprule
            Имя & $ \alpha $ (J2000.0) & $ \delta $ (J2000.0) & $ \varpi $ & $ \sigma_\varpi $ & $ \mu_x $ & $ \sigma_{\mu_x} $ & $ \mu_y $ & $ \sigma_{\mu_y} $ & $ V_\text{LSR} $ & $ \sigma_{V_\text{LSR}} $ & Тип & Источник & Сноски \\
            & (${}^\text{h}$ ${}^\text{m}$ ${}^\text{s}$) & ($\degree$ ${}'$ ${}''$) & \multicolumn{2}{c}{(мсд)} & \multicolumn{2}{c}{(мсд/г)} & \multicolumn{2}{c}{(мсд/г)} & \multicolumn{2}{c}{(км/с)} & & & \\
            \midrule"""
    )

    for object in input_data
        if object.source == "New"
            line = "    "

            name = replace(object.name, "-" => raw"$-$")
            name = replace(name, "+" => raw"$+$")
            line *= "$(name) & "

            alpha = replace(object.alpha, ":" => " ")
            line *= "$(alpha) & "

            delta = replace(object.delta, ":" => " ")
            if !startswith(delta, "-")
                delta = raw"$+$" * delta
            end
            delta = replace(delta, "-" => raw"$-$")
            line *= "$(delta) & "

            format = Printf.Format("%.3f")
            line *= "$(Printf.format(format, object.par)) & "
            line *= "$(Printf.format(format, object.par_e)) & "

            format = Printf.Format("%.2f")
            line *= "\$ $(Printf.format(format, object.mu_x)) \$ & "
            line *= "$(Printf.format(format, object.mu_x_e)) & "

            format = Printf.Format("%.2f")
            line *= "\$ $(Printf.format(format, object.mu_y)) \$ & "
            line *= "$(Printf.format(format, object.mu_y_e)) & "

            format = Printf.Format("%.1f")
            line *= "\$ $(Printf.format(format, object.v_lsr)) \$ & "
            line *= "$(Printf.format(format, object.v_lsr_e)) & "

            line *= "$(object.type) & "
            line *= "$(object.source) & "
            line *= "$(object.reference) "

            line *= "\\\\"
            println(io, line)
        end
    end

    println(
        io,
        raw"""
            \bottomrule
          \end{tabular}
          \caption{Фрагмент нового каталога, содержащий новые объекты (смотри полную версию в машинном формате).}
          \label{table:catalog}
        \end{sidewaystable}"""
    )
end

input_data = nothing

parallaxes_data = CSV.File(
    joinpath(INPUT_DIR, "HMSFRs", "n = 3", "parallaxes.dat"),
    delim=' ',
    comment="#",
)

open(joinpath(OUTPUT_DIR, "parallaxes.tex"), "w") do io
    println(
        io,
        raw"""
        \begin{table}[H]
          \centering
          \aboverulesep=1ex
          \belowrulesep=1ex
          \renewcommand{\arraystretch}{1.25}
          \begin{tabular}{lcccc}
            \toprule
            Имя & $ \varpi $ & $ \sigma_\varpi $ & $ \varpi_0 $ & Источник \\
            & \multicolumn{3}{c}{(мсд)} & \\
            \midrule"""
    )

    for i in 1:41
        object = parallaxes_data[i]

        line = "    "

        name = replace(object.name, "-" => raw"$-$")
        name = replace(name, "+" => raw"$+$")
        line *= "$(name) & "

        format = Printf.Format("%.3f")
        line *= "$(Printf.format(format, object.par)) & "
        line *= "$(Printf.format(format, object.par_e)) & "

        format = Printf.Format("%.5f")
        line *= "$(Printf.format(format, object.par_r)) & "

        line *= "$(object.source) "

        line *= "\\\\"
        println(io, line)
    end

    println(
        io,
        raw"""
            \bottomrule
          \end{tabular}
          \caption{Фрагмент каталога приведенных параллаксов для выборки HMSFRs, $ n = 3 $ (смотри полную версию в машинном формате).}
          \label{table:catalog}
        \end{table}"""
    )
end

parallaxes_data = nothing

println()
