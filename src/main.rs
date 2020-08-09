use roux::Reddit;

static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";
static USER_AGENT: &str = "Vyom by /u/DeltaManiac";

#[tokio::main]
async fn main() {
    let client = Reddit::new(USER_AGENT, CLIENT_ID, CLIENT_SECRET)
        .username("")
        .password("")
        .login()
        .await
        .expect("Dcould login");

    let me = client;
    match me.unread().await {
        Ok(listing) => {
            for i in listing.data.children.iter() {
                     dbg!(&i.data);
                     me.mark_read(&i.data.name).await.expect("Couldnt mark as read");
                // if i.data.r#type == "username_mention" && i.data.new {
                //     dbg!(&i.data);
                //     me.comment("Thats fun3", &i.data.name.as_str())
                //         .await
                //         .expect("Couldnt Comment");
                // }
            }
        }
        Err(e) => {}
    }
}
