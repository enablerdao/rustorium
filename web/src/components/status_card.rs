use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub title: String,
    pub value: String,
    pub icon: String,
    pub color: Option<String>,
}

#[function_component(StatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let color = props.color.clone().unwrap_or_else(|| "primary".to_string());
    
    html! {
        <div class="card stats-card">
            <div class="card-body">
                <div class={format!("text-{} mb-2", color)}>
                    <i class={format!("bi bi-{} fs-3", props.icon)}></i>
                </div>
                <div class="value">{&props.value}</div>
                <div class="label">{&props.title}</div>
            </div>
        </div>
    }
}