use yew::prelude::*;

use crate::components::dropdown::Dropdown;
use crate::helpers::words::Languages;

#[derive(Copy, Clone, PartialEq)]
pub enum LanguageType {
    source,
    target,
}

impl LanguageType {
    fn to_str(&self) -> &str {
        match *self {
            LanguageType::source => "Source",
            LanguageType::target => "Target",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LanguageDropdownProps {
    pub language_type: LanguageType,
    pub value: Languages,
    pub set_value: Callback<Languages>,
}

#[function_component]
pub fn LanguageDropdown(props: &LanguageDropdownProps) -> Html {
    let LanguageDropdownProps {
        language_type,
        value,
        set_value,
    } = props;

    let id = format!("{}-lang-select", language_type.to_str().to_lowercase());
    let label = format!("{} Language", language_type.to_str());
    let options: Vec<String> = vec!["english".to_string(), "german".to_string()];
    let selected = format!("{}", value.to_str());
    let set_value = set_value.clone();
    let on_select = Callback::from(move |lang: String| set_value.emit(Languages::from_str(lang)));

    html! {
        <div class="grid-container">
            <Dropdown
                id={id}
                label={label}
                options={options}
                selected={selected}
                on_select={on_select}
            />
        </div>
    }
}
