pub mod api_actions {

    use rocket::serde::json::Json;
    use serde::Serialize;
    use tokio::task;

    use crate::libs::{other_data::other_data::get_all_cities, processing::processing::check_alarm};

    ///Structure for storing data returned by a request
    #[derive(Debug, Serialize)]
    pub struct ApiResponse {
        pub location_uid: i32,
        pub location_name: String,
        pub is_active_air_alarm: bool
    }


    /// Method for getting alarm information in the specified region.
    /// The return value is serialized to JSON.
    /// The function returns a JSON with the data.
    pub async fn response_builder_for_one_location(location_uid: i32) -> Json<ApiResponse> {

        let alert_status = check_alarm(location_uid).await;
        let all_cities = get_all_cities();
        let basic_location_info = all_cities.get(&location_uid).unwrap();

        let result = ApiResponse {
            location_uid,
            location_name: basic_location_info.to_string(),
            is_active_air_alarm: alert_status,
        };

        Json(result)
    }

    /// Method for getting alarm information in some specified regions.
    /// The return value is serialized to JSON.
    /// The function returns a JSON with the data.
    pub async fn response_builder_for_some_locations(locaion_uids: Vec<i32>) -> Json<Vec<ApiResponse>> {
        
        let data = {
            let cities = get_all_cities();
            let mut _buf = Vec::<ApiResponse>::new();

            for _i in locaion_uids {

                let value = task::spawn(async move {
                    let is_alarm = check_alarm(_i).await;
                    is_alarm
                }).await.unwrap();
                
                let region_name = cities.get(&_i).unwrap().clone();
                let response_data = ApiResponse {
                    location_uid: _i,
                    location_name: region_name,
                    is_active_air_alarm: value
                };
                _buf.push(response_data);

            }
            _buf
        };

        Json(data)
    }

    


}