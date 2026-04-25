use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub on_close: Callback<()>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_close = props.on_close.clone();
    let close = Callback::from(move |_| on_close.emit(()));

    html! {
        <div class="modal-overlay" onclick={close.clone()}>
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="modal__header">
                    <h2 class="modal__title">{ &props.title }</h2>
                    <button class="modal__close" onclick={close}>{ "×" }</button>
                </div>
                <div class="modal__body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
