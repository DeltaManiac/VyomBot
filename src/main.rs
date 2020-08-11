#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log; // Used for logging
use env_logger::Env;
use reqwest;
use roux::Reddit;
use serde_json;

#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    match Reddit::new(
        dotenv!("VYOM_USERAGENT"),
        dotenv!("VYOM_CLIENT_ID"),
        dotenv!("VYOM_CLIENT_SECRET"),
    )
    .username(dotenv!("VYOM_USERNAME"))
    .password(dotenv!("VYOM_PASSWORD"))
    .login()
    .await
    {
        // Try to make a new client with the credentials
        Ok(client) => match client.unread().await {
            // Fetch only the unread messages form the inbox of the logged in user
            Ok(listing) => {
                for message in listing.data.children.iter() {
                    // We have removed the `new` check
                    if message.data.r#type == "username_mention" {
                        let post_url = format!(
                            "https://www.reddit.com/{}/.json",
                            message
                                .data
                                .context
                                .trim()
                                .split('/')
                                .skip(1)
                                .collect::<Vec<&str>>()[0..=4]
                                .join("/")
                        );
                        // let playlist_id = match reqwest::get(&post_url).await {
                        //     Ok(response) => match response.json::<serde_json::Value>().await {
                        //         Ok(json) => {
                        //             let url = match json
                        //                 .get(0)
                        //                 .unwrap()
                        //                 .get("data")
                        //                 .unwrap()
                        //                 .get("children")
                        //                 .unwrap()
                        //                 .get(0)
                        //                 .unwrap()
                        //                 .get("data")
                        //                 .unwrap()
                        //                 .get("url")
                        //             {
                        //                 Some(url) => match url::Url::parse(
                        //                     &url.to_string().trim_matches('\"'),
                        //                 ) {
                        //                     Ok(url) => {
                        //                         match (
                        //                             url.query_pairs().find(|q| {
                        //                                 // dbg!(q);
                        //                                 q.0 == "list"
                        //                             }),
                        //                             (url.host_str() == Some("youtube.com")
                        //                                 || url.host_str()
                        //                                     == Some("www.youtube.com")),
                        //                         ) {
                        //                             (Some((_, val)), true) => {
                        //                                 Some(val.into_owned())
                        //                             }
                        //                             (_, _) => {
                        //                                 error!(
                        //                                     "Couldnt find `list` param in url {} for message : {}",
                        //                                     &url.to_string(),
                        //                                     &message.data.name
                        //                                 );
                        //                                 None
                        //                             }
                        //                         }
                        //                     }
                        //                     Err(e) => {
                        //                         error!(
                        //                             "Couldn't parse url
                        //                             : {} for id :{} reason :  {:?}",
                        //                             &url.to_string(),
                        //                             &message.data.name,
                        //                             e
                        //                         );
                        //                         None
                        //                     }
                        //                 },
                        //                 None => {
                        //                     error!(
                        //                         "Couldn't find url parameter for comment {}",
                        //                         &message.data.name
                        //                     );
                        //                     None
                        //                 }
                        //             };
                        //             url
                        //         }
                        //         Err(e) => {
                        //             error!(
                        //                 "Couldn't parse reponse parameter for comment {} reason : {}",
                        //                 &message.data.name,e
                        //             );
                        //             None
                        //         }
                        //     },
                        //     Err(e) => {
                        //         error!(
                        //             "Couldn't fetch post data for comment {} reason : {}",
                        //             &message.data.name, e
                        //         );
                        //         None
                        //     }
                        // };
                        // Make an http request to the post url
                        let playlist_id = match reqwest::get(&post_url).await {
                            // If the response is received convert it in to dynamic json
                            Ok(response) => match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    // Get json[0]["data"]["children][0]["url}
                                    // NB: DO NOT USE THIS CODE IN PRODUCTION
                                    let url = match json
                                        .get(0)
                                        .unwrap()
                                        .get("data")
                                        .unwrap()
                                        .get("children")
                                        .unwrap()
                                        .get(0)
                                        .unwrap()
                                        .get("data")
                                        .unwrap()
                                        .get("url")
                                    {
                                        // Parse the youtube url from the string "https://www.youtube.com/playlist?list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ" after trimming of `"`
                                        Some(url) => match url::Url::parse(
                                            &url.to_string().trim_matches('\"'),
                                        ) {
                                            Ok(url) => {
                                                match (
                                                    // From the query parameters find the parameter with key like
                                                    url.query_pairs().find(|q| q.0 == "list"),
                                                    // Also check if the host is youtube
                                                    (url.host_str() == Some("youtube.com")
                                                        || url.host_str()
                                                            == Some("www.youtube.com")),
                                                ) {
                                                    (Some((_, val)), true) => {
                                                        // Return the url
                                                        Some(val.into_owned())
                                                    }
                                                    (_, _) => {
                                                        error!(
                                    "Couldn't find `list` param in url {} for message : {}",
                                    &url.to_string(),
                                    &message.data.name
                                );
                                                        None
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                error!(
                                                    "Couldn't parse url 
                            : {} for id :{} reason :  {:?}",
                                                    &url.to_string(),
                                                    &message.data.name,
                                                    e
                                                );
                                                None
                                            }
                                        },
                                        None => {
                                            error!(
                                                "Couldn't find url parameter for comment {}",
                                                &message.data.name
                                            );
                                            None
                                        }
                                    };
                                    url
                                }
                                Err(e) => {
                                    error!(
                                    "Couldn't parse response parameter for comment {} reason : {}",
                                     &message.data.name, e
                                         );
                                    None
                                }
                            },
                            Err(e) => {
                                error!(
                                    "Couldn't fetch post data for comment {} reason : {}",
                                    &message.data.name, e
                                );
                                None
                            }
                        };
                        if playlist_id.is_some() {
                            dbg!("Go fetch youtube");
                        }
                        match client
                            .comment(
                                "Thank you for standing by while we squished a bug. You shouldn't be seeing this message again!",
                                &message.data.name.as_str(),
                            )
                            .await
                        {
                            Ok(_) => {
                                info!("Replied to {}", message.data.name);
                                match client.mark_read(message.data.name.as_str()).await {
                                    Ok(_) => info!("Marked {} as read", message.data.name),
                                    Err(e) => {
                                        error!("Failed to mark {} as read : reason : {:?}", message.data.name, e)
                                    }
                                }
                            }
                            Err(e) => error!("Failed to reply to mention {} : reason : {:?}", message.data.name,e),
                        };
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {:?}", e);
            }
        },
        Err(e) => panic!(e),
    }
}
