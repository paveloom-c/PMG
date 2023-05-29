(function() {var implementors = {
"argmin":[],
"pmg":[["impl&lt;'a, F, FN&gt; <a class=\"trait\" href=\"argmin/core/problem/trait.Gradient.html\" title=\"trait argmin::core::problem::Gradient\">Gradient</a> for <a class=\"struct\" href=\"pmg/model/fit/frozen_outer/struct.FrozenOuterOptimizationProblem.html\" title=\"struct pmg::model::fit::frozen_outer::FrozenOuterOptimizationProblem\">FrozenOuterOptimizationProblem</a>&lt;'a, F, FN&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a> + <a class=\"trait\" href=\"argmin/core/float/trait.ArgminFloat.html\" title=\"trait argmin::core::float::ArgminFloat\">ArgminFloat</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminZeroLike.html\" title=\"trait argmin_math::ArgminZeroLike\">ArgminZeroLike</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;: <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL1Norm.html\" title=\"trait argmin_math::ArgminL1Norm\">ArgminL1Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSignum.html\" title=\"trait argmin_math::ArgminSignum\">ArgminSignum</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMinMax.html\" title=\"trait argmin_math::ArgminMinMax\">ArgminMinMax</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"pmg/utils/finite_diff/trait.FiniteDiff.html\" title=\"trait pmg::utils::finite_diff::FiniteDiff\">FiniteDiff</a>&lt;F&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;FN: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(F, &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.slice.html\">[F]</a>) -&gt; F,</span>"],["impl&lt;'a, F&gt; <a class=\"trait\" href=\"argmin/core/problem/trait.Gradient.html\" title=\"trait argmin::core::problem::Gradient\">Gradient</a> for <a class=\"struct\" href=\"pmg/model/fit/outer/struct.OuterOptimizationProblem.html\" title=\"struct pmg::model::fit::outer::OuterOptimizationProblem\">OuterOptimizationProblem</a>&lt;'a, F&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a> + <a class=\"trait\" href=\"argmin/core/float/trait.ArgminFloat.html\" title=\"trait argmin::core::float::ArgminFloat\">ArgminFloat</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminZeroLike.html\" title=\"trait argmin_math::ArgminZeroLike\">ArgminZeroLike</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;: <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL1Norm.html\" title=\"trait argmin_math::ArgminL1Norm\">ArgminL1Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSignum.html\" title=\"trait argmin_math::ArgminSignum\">ArgminSignum</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMinMax.html\" title=\"trait argmin_math::ArgminMinMax\">ArgminMinMax</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"pmg/utils/finite_diff/trait.FiniteDiff.html\" title=\"trait pmg::utils::finite_diff::FiniteDiff\">FiniteDiff</a>&lt;F&gt;,</span>"],["impl&lt;'a, F&gt; <a class=\"trait\" href=\"argmin/core/problem/trait.Gradient.html\" title=\"trait argmin::core::problem::Gradient\">Gradient</a> for <a class=\"struct\" href=\"pmg/model/fit/sigma_outer/struct.SigmaOuterOptimizationProblem.html\" title=\"struct pmg::model::fit::sigma_outer::SigmaOuterOptimizationProblem\">SigmaOuterOptimizationProblem</a>&lt;'a, F&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a> + <a class=\"trait\" href=\"argmin/core/float/trait.ArgminFloat.html\" title=\"trait argmin::core::float::ArgminFloat\">ArgminFloat</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminZeroLike.html\" title=\"trait argmin_math::ArgminZeroLike\">ArgminZeroLike</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;: <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSub.html\" title=\"trait argmin_math::ArgminSub\">ArgminSub</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminAdd.html\" title=\"trait argmin_math::ArgminAdd\">ArgminAdd</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;F, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminMul.html\" title=\"trait argmin_math::ArgminMul\">ArgminMul</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL1Norm.html\" title=\"trait argmin_math::ArgminL1Norm\">ArgminL1Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminSignum.html\" title=\"trait argmin_math::ArgminSignum\">ArgminSignum</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminMinMax.html\" title=\"trait argmin_math::ArgminMinMax\">ArgminMinMax</a> + <a class=\"trait\" href=\"argmin_math/trait.ArgminDot.html\" title=\"trait argmin_math::ArgminDot\">ArgminDot</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.67.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;F&gt;, F&gt; + <a class=\"trait\" href=\"argmin_math/trait.ArgminL2Norm.html\" title=\"trait argmin_math::ArgminL2Norm\">ArgminL2Norm</a>&lt;F&gt; + <a class=\"trait\" href=\"pmg/utils/finite_diff/trait.FiniteDiff.html\" title=\"trait pmg::utils::finite_diff::FiniteDiff\">FiniteDiff</a>&lt;F&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()