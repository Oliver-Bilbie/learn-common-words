use yew::prelude::*;

use crate::components::game::Game;
use crate::components::language_select::LanguageSelect;
use crate::helpers::words::Languages;

#[function_component]
pub fn App() -> Html {
    enum Form {
        Game,
        LanguageSelect,
    }

    let show_form = use_state(|| Form::LanguageSelect);
    let source_language = use_state(|| Languages::De);
    let target_language = use_state(|| Languages::En);

    let handle_change_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(match *show_form {
                Form::Game => Form::LanguageSelect,
                Form::LanguageSelect => Form::Game,
            });
        })
    };

    let set_source_language = {
        let source_language = source_language.clone();
        Callback::from(move |lang: Languages| {
            source_language.set(lang);
        })
    };

    let set_target_language = {
        let target_language = target_language.clone();
        Callback::from(move |lang: Languages| {
            target_language.set(lang);
        })
    };

    html! {
        <div id="app">
            <div class="box">
                {match *show_form {
                    Form::Game => html! {
                        <Game
                            source_language={*source_language}
                            target_language={*target_language}
                        />
                    },
                    Form::LanguageSelect => html! {
                        <LanguageSelect
                            source_language={*source_language}
                            set_source_language={set_source_language}
                            target_language={*target_language}
                            set_target_language={set_target_language}
                        />
                    },
                }}
                <div class="box">
                    <button class="option_button fade-in" onclick={handle_change_form}>
                        <p>
                            {
                                match *show_form {
                                    Form::Game => "Change languages",
                                    Form::LanguageSelect => "Start game",
                                }
                            }
                        </p>
                    </button>
                </div>
            </div>
        </div>
    }
}
