use leptos::ev::SubmitEvent;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;

async fn fetch_results(search_query: &str) -> Result<Vec<String>, Error> {
    // fetch search results from the server
    let response = reqwest::get(&format!("http://10.0.4.41:7777/movsearch/{}", search_query)).await?;
    let results = response.json::<Vec<String>>().await?;
    Ok(results)
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let search_query = signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().expect("Failed to cast to HtmlInputElement");
        let value = input.value();
        search_query.1.set(value.clone());

        spawn_local(async move {
            match fetch_results(&value).await {
                Ok(results) => {
                    // handle results
                    log::info!("Search results: {:?}", results);
                }
                Err(err) => {
                    // handle error
                    log::error!("Error fetching search results: {:?}", err);
                }
            }
        });
    };

    view! {
        <div class="searchDiv">
            <div class="searchInnerDiv">
                <form method="GET" action="" on:submit=on_submit>
                    <input class="searchInput" type="text" placeholder="Search..." />
                    <button class="searchButton" type="submit">Submit</button>
                </form>
            </div>
            <span class="spacerSpan"></span>
            <div class="searchResults">
                <div class="searchResultDiv">
                    <img src="https://apod.nasa.gov/apod/image/2503/LunarEclipseColors_Jin_960.jpg" alt="suppose to be a pic" />
                    // <span>Movie Title</span>
                </div>
            </div>
        </div>
    }
}

