(function() {var implementors = {
"argmin":[],
"pmg":[["impl&lt;O, L, P, G, F&gt; <a class=\"trait\" href=\"argmin/core/solver/trait.Solver.html\" title=\"trait argmin::core::solver::Solver\">Solver</a>&lt;O, <a class=\"struct\" href=\"argmin/core/state/iterstate/struct.IterState.html\" title=\"struct argmin::core::state::iterstate::IterState\">IterState</a>&lt;P, G, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>, F&gt;&gt; for <a class=\"struct\" href=\"pmg/model/fit/steepest_descent/struct.SteepestDescent.html\" title=\"struct pmg::model::fit::steepest_descent::SteepestDescent\">SteepestDescent</a>&lt;L, F&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"argmin/core/problem/trait.CostFunction.html\" title=\"trait argmin::core::problem::CostFunction\">CostFunction</a>&lt;Param = P, Output = F&gt; + <a class=\"trait\" href=\"argmin/core/problem/trait.Gradient.html\" title=\"trait argmin::core::problem::Gradient\">Gradient</a>&lt;Param = P, Gradient = G&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"argmin/core/serialization/trait.SerializeAlias.html\" title=\"trait argmin::core::serialization::SerializeAlias\">SerializeAlias</a> + <a class=\"trait\" href=\"argmin/core/serialization/trait.DeserializeOwnedAlias.html\" title=\"trait argmin::core::serialization::DeserializeOwnedAlias\">DeserializeOwnedAlias</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, P&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"argmin/core/serialization/trait.SerializeAlias.html\" title=\"trait argmin::core::serialization::SerializeAlias\">SerializeAlias</a> + <a class=\"trait\" href=\"argmin/core/serialization/trait.DeserializeOwnedAlias.html\" title=\"trait argmin::core::serialization::DeserializeOwnedAlias\">DeserializeOwnedAlias</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, P&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"argmin/solver/linesearch/trait.LineSearch.html\" title=\"trait argmin::solver::linesearch::LineSearch\">LineSearch</a>&lt;P, F&gt; + <a class=\"trait\" href=\"argmin/core/solver/trait.Solver.html\" title=\"trait argmin::core::solver::Solver\">Solver</a>&lt;O, <a class=\"struct\" href=\"argmin/core/state/iterstate/struct.IterState.html\" title=\"struct argmin::core::state::iterstate::IterState\">IterState</a>&lt;P, G, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>, F&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"argmin/core/float/trait.ArgminFloat.html\" title=\"trait argmin::core::float::ArgminFloat\">ArgminFloat</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()