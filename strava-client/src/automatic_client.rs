use crate::data_struct::OrdersCancelingSettings;
use crate::request_builder::RequestBuilder;

pub struct AutomaticStravaClient {
    request_builder: RequestBuilder,
    settings: OrdersCancelingSettings,
}
impl AutomaticStravaClient {
    pub fn new(settings: OrdersCancelingSettings) -> Result<AutomaticStravaClient, String> {
        Ok(AutomaticStravaClient {
            request_builder: RequestBuilder::new(),
            settings: settings,            
        })
    }
    pub async fn cancel_orders(&self) -> Result<(), String> {
        let menu =  self.request_builder.do_get_user_menu_request().await?;
        menu.iter().for_each(|(date, dishes)| {
            dishes.iter().for_each(|(name, dish)| {
               if self.settings.blacklisted_dishes.contains(&name) {
                   
               }
            });
        });
        Ok(())
    }
}