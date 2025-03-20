use leptos::prelude::*;
// use leptos::use_query_map;
// use leptos::html::Form;

// async fn fetch_results() {

// }

#[component]
pub fn SearchPage() -> impl IntoView {
    // reactive access to URL query strings
    // let query = use_query_map();
    // // search stored as ?q=
    // let search = move || query.read().get("q").unwrap_or_default();
    // // a resource driven by the search string
    // let search_results = Resource::new(search, |_| fetch_results());

    view! {
        <div class="searchDiv">
            <div class="searchInnerDiv">
		        <form method="GET" action="">
                	<input class="search-input" type="text" placeholder="Search..." />
                	// <button class="searchButton">Search</button>
			        <input class="searchButton" type="submit" />
		        </form>
            </div>
            <span></span>
            <span></span>
        </div>
    }
}