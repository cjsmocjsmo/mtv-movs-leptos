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
async fn fetch_pandas() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/pandas").await?;
    let pandas: Vec<Infos> = response.json().await?;
    Ok(pandas)
}
async fn fetch_pirates() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/pirates").await?;
    let pirates: Vec<Infos> = response.json().await?;
    Ok(pirates)
}
async fn send_get_request(mov_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/player_set_media/{}", mov_id);
    reqwest::get(&url).await?;
    Ok(())
}

#[component]
pub fn PandasPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_pandas().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="movRow">
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
        <div class="image-placeholder"></div>
    }
}


#[component]
pub fn PiratesPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_pirates().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="movRow">
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
        <div class="image-placeholder"></div>
    }
}
