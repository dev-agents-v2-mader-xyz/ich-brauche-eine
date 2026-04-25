use crate::types::DrinkType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DrinkButtonProps {
    pub drink: DrinkType,
    pub on_click: Callback<String>,
}

#[function_component(DrinkButton)]
pub fn drink_button(props: &DrinkButtonProps) -> Html {
    let id = props.drink.id.clone();
    let on_click = props.on_click.clone();
    let onclick = Callback::from(move |_| on_click.emit(id.clone()));

    html! {
        <button class="drink-btn" {onclick}>
            <span class="drink-btn__emoji">{ &props.drink.emoji }</span>
            <span class="drink-btn__name">{ &props.drink.name }</span>
            <span class="drink-btn__mg">{ format!("{} mg", props.drink.caffeine_mg) }</span>
        </button>
    }
}
