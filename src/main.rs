use libs::{api_actions::api_actions::{response_builder_for_one_location, response_builder_for_some_locations, ApiResponse}, other_data::other_data::{get_all_cities_as_json, RegionInfo}, processing::processing::update_data};
use std::collections::HashMap;
use rocket::{
    serde::json::{json, Json, Value},
    catch, catchers, get, routes
};

mod libs {
    pub mod fetch_data;
    pub mod processing;
    pub mod data_actions;
    pub mod other_data;
    pub mod api_actions;
}

#[get("/get_alarm/<id>")]
async fn get_alarm_from_one_region(id: i32) -> Json<ApiResponse> {

    let response = response_builder_for_one_location(id).await;
    response

}
#[get("/get_alarms?<params..>")]
async fn get_alarm_from_some_regions(params: HashMap<String, i32>) -> Json<Vec<ApiResponse>> {

    let mut location_ids = Vec::<i32>::new();
    for (_, value) in params.iter() {
        location_ids.push(*value);
    }

    let response = response_builder_for_some_locations(location_ids).await;
    response
}

#[get("/get_regions")]
async fn get_info_about_available_regions() -> Json<Vec<RegionInfo>> {
    let locations = get_all_cities_as_json();
    locations
}

#[catch(404)]
fn error_404() -> Value {
    json!("THE LOCATION NOT FOUND")
}

#[rocket::main]
async fn main() {

    tokio::spawn(async {
        loop {
            update_data().await;
        }
    });

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_alarm_from_one_region,
                get_alarm_from_some_regions,
                get_info_about_available_regions
            ],
        )
        .register("/", catchers![error_404])
        .launch()
        .await;

}