use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownProps {
    pub id: String,
    pub label: String,
    pub options: Vec<String>,
    pub selected: String,
    pub on_select: Callback<String>,
}

#[function_component]
pub fn Dropdown(props: &DropdownProps) -> Html {
    let DropdownProps {
        id,
        label,
        options,
        selected,
        on_select,
    } = props.clone();

    let show_options = use_state(|| false);

    let handle_click = {
        let show_options = show_options.clone();
        move |_| show_options.set(!*show_options)
    };

    let handle_selection = |option: String| {
        let on_select = on_select.clone();
        let show_options = show_options.clone();

        move |_| {
            on_select.emit(option.clone());
            show_options.set(false);
        }
    };

    html! {
         <div id={format!("{}", id)} class="dropdown">
            <h4>{ label }</h4>
            <button class="selection-button" onclick={handle_click}>{ selected }</button>
            if *show_options {
            <div class="dropdown-content">
                {
                    for options.iter().map(|option| html! {
                        <a
                            href="#"
                            onclick={handle_selection(option.to_string())}
                        >
                            {option}
                        </a>
                    })
                }
            </div>}
        </div>
    }
}
