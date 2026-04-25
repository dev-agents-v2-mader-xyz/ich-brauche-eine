use crate::types::DrinkEntry;
use crate::utils::format_time;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogEntryProps {
    pub entry: DrinkEntry,
    pub on_delete: Callback<String>,
}

#[function_component(LogEntry)]
pub fn log_entry(props: &LogEntryProps) -> Html {
    let id = props.entry.id.clone();
    let on_delete = props.on_delete.clone();
    let onclick = Callback::from(move |_| on_delete.emit(id.clone()));

    html! {
        <div class="log-entry">
            <span class="log-entry__emoji">{ &props.entry.drink_type.emoji }</span>
            <div class="log-entry__info">
                <span class="log-entry__name">{ &props.entry.drink_type.name }</span>
                <span class="log-entry__time">{ format_time(&props.entry.consumed_at) }</span>
            </div>
            <span class="log-entry__mg">{ format!("{} mg", props.entry.drink_type.caffeine_mg) }</span>
            <button class="log-entry__delete" {onclick} aria-label="Löschen">{ "×" }</button>
        </div>
    }
}
