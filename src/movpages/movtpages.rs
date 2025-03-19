#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Infos {
    Name: String,
    HttpThumbPath: String,
    MovId: String,
}
async fn fetch_tinkerbell() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tinkerbell").await?;
    let tinkerbell: Vec<Infos> = response.json().await?;
    Ok(tinkerbell)
}
async fn fetch_tomcruise() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tomcruise").await?;
    let tomcruise: Vec<Infos> = response.json().await?;
    Ok(tomcruise)
}
async fn fetch_transformers() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/transformers").await?;
    let transformers: Vec<Infos> = response.json().await?;
    Ok(transformers)
}
async fn fetch_tremors() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/tremors").await?;
    let tremors: Vec<Infos> = response.json().await?;
    Ok(tremors)
}
async fn fetch_therock() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/therock").await?;
    let therock: Vec<Infos> = response.json().await?;
    Ok(therock)
}
async fn send_get_request(mov_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.777:8080/player_set_media/{}", mov_id);
    reqwest::get(&url).await?;
    Ok(())
}

#[component]
pub fn TinkerBellPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tinkerbell().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {let infos = move || infos.get().clone(); move || infos().iter().map(|info| {
                let info = info.clone();
                view! {
                    <img 
                        src={info.HttpThumbPath.clone()} 
                        alt={info.Name.clone()}
                        on:click=move |_| {
                            let mov_id = info.MovId.clone();
                            spawn_local(async move {
                                if let Err(err) = send_get_request(&mov_id).await {
                                    log::error!("Error sending GET request: {:?}", err);
                                }
                            });
                        }
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn TomCruisePage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tomcruise().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {let infos = move || infos.get().clone(); move || infos().iter().map(|info| {
                let info = info.clone();
                view! {
                    <img 
                        src={info.HttpThumbPath.clone()} 
                        alt={info.Name.clone()}
                        on:click=move |_| {
                            let mov_id = info.MovId.clone();
                            spawn_local(async move {
                                if let Err(err) = send_get_request(&mov_id).await {
                                    log::error!("Error sending GET request: {:?}", err);
                                }
                            });
                        }
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn TransformersPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_transformers().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {let infos = move || infos.get().clone(); move || infos().iter().map(|info| {
                let info = info.clone();
                view! {
                    <img 
                        src={info.HttpThumbPath.clone()} 
                        alt={info.Name.clone()}
                        on:click=move |_| {
                            let mov_id = info.MovId.clone();
                            spawn_local(async move {
                                if let Err(err) = send_get_request(&mov_id).await {
                                    log::error!("Error sending GET request: {:?}", err);
                                }
                            });
                        }
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn TremorsPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_tremors().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {let infos = move || infos.get().clone(); move || infos().iter().map(|info| {
                let info = info.clone();
                view! {
                    <img 
                        src={info.HttpThumbPath.clone()} 
                        alt={info.Name.clone()}
                        on:click=move |_| {
                            let mov_id = info.MovId.clone();
                            spawn_local(async move {
                                if let Err(err) = send_get_request(&mov_id).await {
                                    log::error!("Error sending GET request: {:?}", err);
                                }
                            });
                        }
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn TheRockPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_therock().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {let infos = move || infos.get().clone(); move || infos().iter().map(|info| {
                let info = info.clone();
                view! {
                    <img 
                        src={info.HttpThumbPath.clone()} 
                        alt={info.Name.clone()}
                        on:click=move |_| {
                            let mov_id = info.MovId.clone();
                            spawn_local(async move {
                                if let Err(err) = send_get_request(&mov_id).await {
                                    log::error!("Error sending GET request: {:?}", err);
                                }
                            });
                        }
                    />
                }
            }).collect_view()}
        </div>
    }
}
