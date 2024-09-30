
pub mod fetch_data {

    use data_views::AlertsResponseResult;
    use dotenv::dotenv;
    use std::env;
    use reqwest::StatusCode;
    
    // Getting a copy of the connection string to the alarm API
    fn get_alerts_api_connection_url() -> String {
        dotenv().ok();
    
        let token = env::var("TOKEN").expect("INCORRECT ALERT API TOKEN");
        let url = format!(
                "https://api.alerts.in.ua/v1/alerts/active.json?token={}",
                token
        );
        url
    }
    
    /// Function to get data, where :
    ///
    /// - last_modified - header from API response
    ///     contains data when alarm data was last updated on the server
    ///
    /// - the return value is of type Result<(AlertsResponseResult, String)>
    ///      the return value is a tuple of two elements:
    ///         - struct AlertsResponseResult
    ///         - String (the response header value "If-Modified-Since" is returned as a string)
    ///
    pub async fn get_alerts(last_modified: String) -> Result<(AlertsResponseResult, String), Box<dyn std::error::Error>> {

        // Getting a copy of the connection string to the alarm API
        let url = get_alerts_api_connection_url();

        // Create a client to make an HTTP request
        let client = reqwest::Client::new();

        /* 
            Sending request using the header "If-Modified-Since",
            in order not to start data processing if there were
            no changes on the server and there is no new data.
        */
        let response = client.get(&url)
            .header("If-Modified-Since", last_modified.to_string())
            .send()
            .await?;

        // Processing the query result
        if response.status() == StatusCode::NOT_MODIFIED {
            return Err("No new data available (304 Not Modified)".into());
        }

        // Getting all response headers
        let updated_at = response.headers().clone();

        //Deserializing data to AlertsResponseResult type
        let result = response.json::<AlertsResponseResult>().await?;

        if let Some(header_value) = updated_at.get("last-modified") {

            // Getting the "last-modified" header value
            if let Ok(header) = header_value.to_str() {

                // Returning the result of a method as a tuple
                return Ok((result, header.to_string()));
            }
        }
        Err("Failed to process the response headers".into())



    }


    pub mod data_views {

        /// General structure of data received from the API alarms
        #[derive(Debug, serde::Deserialize)]
        pub struct AlertsResponseResult{
            pub alerts: Vec<Location>,
            meta: Option<Meta>,
            disclaimer: Option<String>
        }

        /// Meta structure is part of the data received
        /// from the official alarm API
        #[derive(Debug, serde::Deserialize)]
        struct Meta{
            last_updated_at: Option<String>,
            meta_type: Option<String>
        }
    
        /// Location data
        #[derive(Debug, serde::Deserialize, Clone)]
        pub struct Location {
            pub id: i32,
            pub location_title: String,
            pub location_type: String,
            pub started_at: String,
            pub finished_at: Option<String>,
            pub updated_at: Option<String>,
            pub alert_type: String,
            pub location_uid: Option<String>,
            pub location_oblast: Option<String>,
            pub location_oblast_uid: Option<i16>,
            pub location_raion: Option<String>,
            pub notes: Option<String>,
            pub calculated: Option<bool>,
        }

    }
}