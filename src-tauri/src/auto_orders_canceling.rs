use crate::request_builder::RequestBuilder;
struct OrdersCanceling {
    client: RequestBuilder,
}
impl OrdersCanceling {
    pub fn new() -> Result<OrdersCanceling, String> {
        Ok(OrdersCanceling {
            client: RequestBuilder::new(),
        })
    }
    /*
    pub async fn fetch_settings(&self) -> Result<(), String> {
                                                                    //TODO fetch settings from db
    }
    */
    
    pub async fn cancel_orders(&self) {}
}
#[tokio::main]
async fn main() {
    keytar::set_password("strava_client", "username", "password").unwrap();
    keytar::set_password("strava_client", "username1", "password1").unwrap();
    let x = keytar::find_password("strava_client").unwrap();
    println!("{}", x.password);
}
