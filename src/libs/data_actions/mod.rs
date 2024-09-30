
pub mod data_actions {

    use std::collections::HashSet;
    use log::debug;
    use crate::libs::fetch_data::fetch_data::get_alerts;
    use crate::libs::other_data::other_data::get_all_cities;


    /// Struct fo representing info about of the alarm data
    #[derive(Debug, Clone)]
    pub struct CurrentAlarm {
        pub _location_uid: String,
        pub _location_oblast_uid: i16,
        pub _location_title: String,
        pub _location_oblast: String,
        pub _started_at: String,
    }

    /// A function that takes as input an AlertsResponseResult structure
    /// that contains active alarms and metadata,
    /// and a string that stores the value received
    /// from the "last-modified" header.
    /// The function returns a vector of CurrentAlarm structures
    /// that represent the deserialized JSON
    /// and the value from the "last-modified" header.
    pub async fn deserialize_current_alarms_data(updated_at: String) -> Result<(Vec<CurrentAlarm>, String), String> {

        //Getting current alarm's data
        let current_alarms_data = get_alerts(updated_at.clone()).await;

        //Getting IDs of all regions
        let city_ids: HashSet<i32> = get_all_cities().keys().cloned().collect();
        let result_data: (Vec<CurrentAlarm>, String);

        let result: Result<(Vec<CurrentAlarm>, String), String> = match current_alarms_data {
            Ok(value) => {

                debug!("{:?} - the current alarm data is available", chrono::Local::now());

                let (alarms, header) = (value.0.alerts, value.1);
                let deserialized_alarms = {

                    let mut collected_data = Vec::<CurrentAlarm>::new();
                    debug!("{:?} - starting deserializing", chrono::Local::now());

                    for v in alarms { 

                        /*
                            Sorting data by alarm type and checking whether
                            the ID passed to the function is in the general list
                            of available locations.
                            (Periodically, the API passes IDs of locations and regions
                            that have not yet been presented in their official documentation).
                        */

                        if (v.alert_type == String::from("air_raid")) && city_ids.contains(&(v.location_oblast_uid.unwrap() as i32)) {

                            /*
                                If the check is successful, the data is generated
                                in the CurrentAlarm structure.
                            */
                            let compact_data = CurrentAlarm{
                                _location_uid: v.location_uid.unwrap(),
                                _location_oblast_uid: v.location_oblast_uid.unwrap(),
                                _location_title: v.location_title,
                                _started_at: v.started_at,
                                _location_oblast: v.location_oblast.unwrap(),
                            };

                            debug!("{:?} - deserialized location data:\n{:?}", chrono::Local::now(), compact_data);
                            collected_data.push(compact_data);

                        }else{

                            debug!("{:?} - not deserialized location data:\n{:?}", chrono::Local::now(), v);
                        }

                    }
                    collected_data

                };

                //Returning the tuple
                result_data = (deserialized_alarms, header);
                Ok(result_data)

            },
            Err(e) => {

                debug!("{:?} - deserialize data error\n{:?}", chrono::Local::now(), e);

                let err = String::from(format!("{:?} - deserialize data error {:?}", chrono::Local::now(), e));
                Err(err.into())
            }
        };

        return result



    }
}