use yew::prelude::*;

use crate::components::dropdown::Dropdown;
use crate::helpers::words::Languages;

#[derive(Properties, PartialEq)]
pub struct LanguageSelectProps {
    pub source_language: Languages,
    pub set_source_language: Callback<Languages>,
    pub target_language: Languages,
    pub set_target_language: Callback<Languages>,
}

#[function_component]
pub fn LanguageSelect(props: &LanguageSelectProps) -> Html {
    let LanguageSelectProps {
        source_language,
        set_source_language,
        target_language,
        set_target_language,
    } = props;

    let options: Vec<String> = vec![
        Languages::En.to_str().to_string(),
        Languages::De.to_str().to_string(),
        Languages::Fr.to_str().to_string(),
        Languages::It.to_str().to_string(),
        Languages::Es.to_str().to_string(),
    ];
    let selected_src = format!("{}", source_language.to_str());
    let selected_trg = format!("{}", target_language.to_str());
    let set_source_language = set_source_language.clone();
    let set_target_language = set_target_language.clone();
    let on_select_src =
        Callback::from(move |lang: String| set_source_language.emit(Languages::from_str(lang)));
    let on_select_trg =
        Callback::from(move |lang: String| set_target_language.emit(Languages::from_str(lang)));

    html! {
        <div class="box fade-in">
            <Dropdown
                id={"src-lang-select"}
                label={"Source Language"}
                options={options.clone()}
                selected={selected_src}
                on_select={on_select_src}
            />
            <Dropdown
                id={"trg-lang-select"}
                label={"Target Language"}
                options={options.clone()}
                selected={selected_trg}
                on_select={on_select_trg}
            />
        </div>
    }
}
