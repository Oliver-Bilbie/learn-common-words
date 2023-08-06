use yew::prelude::*;

use crate::components::progress_bar::ProgressBar;
use crate::helpers::random::get_random_integers;
use crate::helpers::words::{get_words, Languages};

#[derive(Copy, Clone)]
struct Progress {
    total_points: i32,
    word_limit: i32,
    round_percentage: i32,
}

impl Progress {
    fn new(total_points: i32) -> Progress {
        Progress {
            total_points,
            word_limit: if total_points < 380 {
                50 + 25 * (total_points / 10)
            } else {
                1000
            },
            round_percentage: if total_points < 380 {
                total_points % 10 * 10
            } else {
                100
            },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub source_language: Languages,
    pub target_language: Languages,
}

#[function_component]
pub fn Game(props: &GameProps) -> Html {
    let GameProps {
        source_language,
        target_language,
    } = props;

    let source_language = source_language.clone();
    let target_language = target_language.clone();

    let progress = use_state(|| Progress::new(0));
    let words = use_state(|| get_words(source_language, target_language, 50));
    let order = use_state(|| get_random_integers(3));

    let submit_answer = |answer: String| {
        let progress = progress.clone();
        let words = words.clone();
        let order = order.clone();

        move |_| {
            let updated_points;
            if answer == words.target_correct {
                updated_points = progress.total_points + 1;
            } else {
                updated_points = if progress.total_points > 5 {
                    progress.total_points - 5
                } else {
                    0
                };
            }

            let updated_progress = Progress::new(updated_points);
            progress.set(updated_progress);
            words.set(get_words(
                source_language,
                target_language,
                updated_progress.word_limit.into(),
            ));
            order.set(get_random_integers(3));
        }
    };

    html! {
        <div class="grid-container fade-in">
            <ProgressBar
                id="progress-bar"
                text_progress={progress.total_points.to_string()}
                bar_progress={progress.round_percentage}
            />
            <h1 id="word-prompt">{ &words.source }</h1>
            <button
                id={format!("button-{}", order[0])}
                class="selection-button"
                onclick={submit_answer(words.target_correct.to_string())}
            >
                { &words.target_correct }
            </button>
            <button
                id={format!("button-{}",
                order[1])} class="selection-button"
                onclick={submit_answer(words.target_incorrect_1.to_string())}
            >
                { &words.target_incorrect_1 }
            </button>
            <button
                id={format!("button-{}", order[2])}
                class="selection-button"
                onclick={submit_answer(words.target_incorrect_2.to_string())}
            >
                { &words.target_incorrect_2 }
            </button>
        </div>
    }
}
