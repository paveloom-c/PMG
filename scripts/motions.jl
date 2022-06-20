# Convert proper motions in Galactic coordinates
# to proper motions in the equatorial coordinates

using Measurements

"Convert an HMS angle to radians"
function hms2rad(h, m, s)
    return deg2rad(h * 15 + m / 4 + s / 240)
end

"Convert a DMS angle to radians"
function dms2rad(d, m, s)
    return deg2rad(sign(d) * (abs(d) + m / 60 + s / 3600))
end

α_NGP = hms2rad(12., 51., 26.2817)
δ_NGP = dms2rad(27., 7., 42.013)
l_NCP = deg2rad(122.932)

"""
Convert the coordinates in the equatorial spherical system
(right ascension and declination) to the coordinates in the
Galactic heliocentric spherical system (longitude and latitude)
"""
function convert_coordinates(α, δ)
    # Convert to the Galactic heliocentric spherical system
    φ = atan(
        cos(δ) * sin(α - α_NGP),
        cos(δ_NGP) * sin(δ) - sin(δ_NGP) * cos(δ) * cos(α - α_NGP),
    )
    b = asin(sin(δ_NGP) * sin(δ) + cos(δ_NGP) * cos(δ) * cos(α - α_NGP))
    l = l_NCP - φ
    return l, b
end

"""
Convert proper motions in Galactic coordinates
to proper motions in the equatorial coordinates

Source: Poleski
"""
function convert_motions(α, δ, μ_l_s, μ_b)
    # Compute the elements of the matrix
    c_1 = sin(δ_NGP) * cos(δ) - cos(δ_NGP) * sin(δ) * cos(α - α_NGP)
    c_2 = cos(δ_NGP) * sin(α - α_NGP)
    # Prepare the inverse matrix
    mi = [c_1 -c_2; c_2 c_1]
    # Compute the proper motions
    motions = mi * transpose([μ_l_s μ_b]) * cos(b)
    return tuple(motions...)
end

# Define the coordinates of Sgr B2
#
# Source: the VERA catalogue
α = hms2rad(17, 47, 20.1817)
δ = dms2rad(-28, 23, 03.889)

# Convert the equatorial coordinates to the spherical Galactic coordinates
l, b = convert_coordinates(α, δ)

# Define the proper motions in Galactic coordinates of Sgr B2
#
# Source: Sakai et al. (2021)
μ_l = -4.17 ± 0.19
μ_l_s = μ_l * cos(b)
μ_b = -0.34 ± 0.13

# Convert them to motions in the equatorial coordinates
μ_α_s, μ_δ = convert_motions(α, δ, μ_l_s, μ_b)

# Print the results
println("""

    Sgr B2:
    l:     $(rad2deg(l))
    b:     $(rad2deg(b))
    μ_α_s: $(μ_α_s)
    μ_δ:   $(μ_δ)
""")
