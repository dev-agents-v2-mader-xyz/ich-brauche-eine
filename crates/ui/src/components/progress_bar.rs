use crate::utils::{caffeine_color, progress_fraction};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub total: i32,
    pub limit: i32,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let pct = (progress_fraction(props.total, props.limit) * 100.0) as u32;
    let color = caffeine_color(props.total, props.limit);
    let bar_class = format!("progress-fill progress-fill--{}", color);

    html! {
        <div class="progress-bar">
            <div class={bar_class} style={format!("width: {}%", pct)} />
        </div>
    }
}
