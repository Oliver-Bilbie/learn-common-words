use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub id: String,
    pub text_progress: String,
    pub bar_progress: i32,
}

#[function_component]
pub fn ProgressBar(props: &ProgressBarProps) -> Html {
    let ProgressBarProps {
        id,
        text_progress,
        bar_progress,
    } = props;
    let under_50 = if bar_progress < &50 { "under" } else { "over" };

    html! {
        <div
            id={format!("{}", id)}
            class={format!("progress-bar progress-bar-{}-50", under_50)}
            style={format!("--progress: {}", bar_progress)}
        >
            <div class="progress-bar-overlay">
                <h1>{ text_progress }</h1>
            </div>
        </div>
    }
}
