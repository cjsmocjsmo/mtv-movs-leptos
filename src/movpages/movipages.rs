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
async fn fetch_indianajones() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/indianajones").await?;
    let indianajones: Vec<Infos> = response.json().await?;
    Ok(indianajones)
}
async fn send_get_request(mov_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.777:8080/player_set_media/{}", mov_id);
    reqwest::get(&url).await?;
    Ok(())
}

#[component]
pub fn IndianaJonesPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_indianajones().await {
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
