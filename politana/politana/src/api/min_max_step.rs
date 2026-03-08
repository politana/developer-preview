use crate::{El, Step, api::attr_style::Attribute};

impl El {
    pub fn min_max_step(
        self,
        min: impl Attribute<f64> + 'static,
        max: impl Attribute<f64> + 'static,
        step: impl Attribute<Step> + 'static
    ) -> Self {
        self.attr("min", move || min.into_function()())
            .attr("max", move || max.into_function()())
            .attr("step", move || step.into_function()())
    }
}
