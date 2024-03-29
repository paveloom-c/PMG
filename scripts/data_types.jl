# This script contains types of the data files
# (as vectors, so one could parse it by pushing the values)

module Types
    using Parameters

    struct ObjectsData{F}
        outlier::Vector{Bool}
        name::Vector{String}
        type::Vector{String}
        source::Vector{String}
        α::Vector{F}
        δ::Vector{F}
        par::Vector{F}
        par_e::Vector{F}
        par_p::Vector{F}
        par_m::Vector{F}
        V_lsr::Vector{F}
        V_lsr_e::Vector{F}
        mu_x::Vector{F}
        mu_x_e::Vector{F}
        mu_y::Vector{F}
        mu_y_e::Vector{F}
        r::Vector{F}
        r_ep::Vector{F}
        r_em::Vector{F}
        r_p::Vector{F}
        r_m::Vector{F}
        l::Vector{F}
        b::Vector{F}
        mu_l_cos_b::Vector{F}
        mu_b::Vector{F}
        R::Vector{F}
        R_ep::Vector{F}
        R_em::Vector{F}
        R_p::Vector{F}
        R_m::Vector{F}
        X::Vector{F}
        X_ep::Vector{F}
        X_em::Vector{F}
        X_p::Vector{F}
        X_m::Vector{F}
        Y::Vector{F}
        Y_ep::Vector{F}
        Y_em::Vector{F}
        Y_p::Vector{F}
        Y_m::Vector{F}
        Z::Vector{F}
        Z_ep::Vector{F}
        Z_em::Vector{F}
        Z_p::Vector{F}
        Z_m::Vector{F}
        Θ::Vector{F}
        Θ_ep::Vector{F}
        Θ_em::Vector{F}
        Θ_p::Vector{F}
        Θ_m::Vector{F}
        θ_evel::Vector{F}
        θ_evel_corrected::Vector{F}
        V_r::Vector{F}
        V_r_e::Vector{F}
        V_l::Vector{F}
        V_l_ep::Vector{F}
        V_l_em::Vector{F}
        V_l_p::Vector{F}
        V_l_m::Vector{F}
        V_b::Vector{F}
        V_b_ep::Vector{F}
        V_b_em::Vector{F}
        V_b_p::Vector{F}
        V_b_m::Vector{F}
        U::Vector{F}
        U_ep::Vector{F}
        U_em::Vector{F}
        U_p::Vector{F}
        U_m::Vector{F}
        V::Vector{F}
        V_ep::Vector{F}
        V_em::Vector{F}
        V_p::Vector{F}
        V_m::Vector{F}
        W::Vector{F}
        W_ep::Vector{F}
        W_em::Vector{F}
        W_p::Vector{F}
        W_m::Vector{F}
    end

    struct FitRotCurveData{F}
        R::Vector{F}
        Θ::Vector{F}
        σ::Vector{F}
    end

    @with_kw struct Params{F}
        R_0::Vector{F} = []
        R_0_ep::Vector{F} = []
        R_0_em::Vector{F} = []
        ω_0::Vector{F} = []
        ω_0_ep::Vector{F} = []
        ω_0_em::Vector{F} = []
        A::Vector{F} = []
        A_ep::Vector{F} = []
        A_em::Vector{F} = []
        u_sun::Vector{F} = []
        u_sun_ep::Vector{F} = []
        u_sun_em::Vector{F} = []
        v_sun::Vector{F} = []
        v_sun_ep::Vector{F} = []
        v_sun_em::Vector{F} = []
        w_sun::Vector{F} = []
        w_sun_ep::Vector{F} = []
        w_sun_em::Vector{F} = []
        σ_R::Vector{F} = []
        σ_R_ep::Vector{F} = []
        σ_R_em::Vector{F} = []
        σ_θ::Vector{F} = []
        σ_θ_ep::Vector{F} = []
        σ_θ_em::Vector{F} = []
        σ_Z::Vector{F} = []
        σ_Z_ep::Vector{F} = []
        σ_Z_em::Vector{F} = []
        θ_2::Vector{F} = []
        θ_2_ep::Vector{F} = []
        θ_2_em::Vector{F} = []
        θ_3::Vector{F} = []
        θ_3_ep::Vector{F} = []
        θ_3_em::Vector{F} = []
        θ_4::Vector{F} = []
        θ_4_ep::Vector{F} = []
        θ_4_em::Vector{F} = []
        θ_5::Vector{F} = []
        θ_5_ep::Vector{F} = []
        θ_5_em::Vector{F} = []
        θ_6::Vector{F} = []
        θ_6_ep::Vector{F} = []
        θ_6_em::Vector{F} = []
        θ_7::Vector{F} = []
        θ_7_ep::Vector{F} = []
        θ_7_em::Vector{F} = []
        θ_8::Vector{F} = []
        θ_8_ep::Vector{F} = []
        θ_8_em::Vector{F} = []
        θ_0::Vector{F} = []
        θ_0_ep::Vector{F} = []
        θ_0_em::Vector{F} = []
        θ_1::Vector{F} = []
        θ_1_ep::Vector{F} = []
        θ_1_em::Vector{F} = []
        θ_sun::Vector{F} = []
        θ_sun_ep::Vector{F} = []
        θ_sun_em::Vector{F} = []
        ω_sun::Vector{F} = []
        ω_sun_ep::Vector{F} = []
        ω_sun_em::Vector{F} = []
    end

    struct Profile{F}
        param::Vector{F}
        cost::Vector{F}
    end
end
