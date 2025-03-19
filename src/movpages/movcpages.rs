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

async fn fetch_cartoons() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/cartoons").await?;
    let cartoons: Vec<Infos> = response.json().await?;
    Ok(cartoons)
}
async fn fetch_charliebrown() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/charliebrown").await?;
    let charliebrown: Vec<Infos> = response.json().await?;
    Ok(charliebrown)
}
async fn fetch_chucknorris() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/chucknorris").await?;
    let chucknorris: Vec<Infos> = response.json().await?;
    Ok(chucknorris)
}
async fn fetch_comedy() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/comedy").await?;
    let comedy: Vec<Infos> = response.json().await?;
    Ok(comedy)
}
async fn send_get_request(mov_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.777:8080/player_set_media/{}", mov_id);
    reqwest::get(&url).await?;
    Ok(())
}

#[component]
pub fn CartoonsPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_cartoons().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            }
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
pub fn CharlieBrownPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_charliebrown().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            }
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
pub fn ChuckNorrisPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_chucknorris().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            }
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
pub fn ComedyPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_comedy().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            }
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
