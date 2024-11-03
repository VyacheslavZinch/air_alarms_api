pub mod processing {

    use dotenv::dotenv;
    use std::env;
    use std::collections::HashSet;
    use std::time::Duration;
    use tokio::sync::Mutex; 
    use std::sync::Arc;
    use tokio::task;
    use redis;
    use redis::{Client, AsyncCommands, RedisResult};
    use log::{debug, info};

    use crate::modules::{data_actions::data_actions::{deserialize_current_alarms_data, CurrentAlarm}, other_data::other_data::get_all_cities};

    ///Getting the Redis-host
    fn get_redis_host() -> String {
        dotenv().ok();
        let host = env::var("REDIS_HOST").expect("INCORRECT REDIS ADDRESS");
        host
    }
    
    ///Getting password for Redis connection
    fn get_redis_passwd() -> String {
        dotenv().ok();
        let pwd = env::var("REDIS_PASSWD").expect("THE REDIS PASSWORD IS UNAVAILABLE");
        pwd
    }

    fn get_redis_usr() -> String {
        dotenv().ok();
        let usr = env::var("REDIS_USR").expect("THE REDIS USER IS UNAVAILABLE");
        usr
    }

    

    /// Function for checking location alarm.
    /// To determine the location, its ID is used.
    pub async fn check_alarm(location_id: i32) -> bool {

        let conn_str = format!("redis://{}:{}@{}:6380/0", get_redis_usr(), get_redis_passwd(), get_redis_host());
        let client = Client::open(conn_str);
        let conn = client.unwrap().get_multiplexed_async_connection().await;

        //Getting value by location_id key.
        let value: RedisResult<Option<String>> = conn.unwrap().hget("regions", location_id).await;
        let result = if value.unwrap().unwrap() == "true" {true} else {false};

        debug!("{} - the current state of alarm on location {:?} is {}", chrono::Local::now(), location_id, result);
        result
    }


    /// Sets the alarm status for a location.
    /// The location ID and the new value to set are passed
    /// as parameters to the function.
    pub async fn set_alarm_status(location_id: i32, new_status: bool) {

        let conn_str = format!("redis://{}:{}@{}:6380/0", get_redis_usr(), get_redis_passwd(), get_redis_host());
        let client = Client::open(conn_str);
        let mut conn = client.unwrap().get_multiplexed_async_connection().await.unwrap();
        let new_status = if new_status { "true".to_string() } else { "false".to_string() };


        // Calling the hset function, which is passed the key "regions",region ID and new alarm value
        let result: RedisResult<()> = conn.hset("regions", location_id, new_status.clone()).await;
        debug!("{} - trying to set new alarm status - {} on location {}", chrono::Local::now(), new_status, location_id);

        // Checking if the new value was set successfully
        match result {
            Ok(_) => {
                debug!("{} - {} - alarm status updated", chrono::Local::now(), location_id);
            }
            Err(e) => {
                debug!("{} - {} - error updating alarm status\n{}", chrono::Local::now(), location_id, e);
            }
        }

    }

    /// The function in an infinite loop
    /// makes a request using the alarm API
    /// and asynchronously updates the data in Redis
    pub async fn update_data() {

        let interval: i32 = (60/9) as i32;

        /*
            Initialization with an empty variable value.
            Later, the value of the "last-modified" header is stored
            in this variable. The value of the variable changes
            when a response containing this header
            is received from the server. 
        */
        let updated_date_time = Arc::new(Mutex::new(String::from("")));
        
        loop {

            // Set the pause of the current thread in the loop
            tokio::time::sleep(Duration::from_secs(interval as u64)).await;
            debug!("{} - getting new data for air alarms", chrono::Local::now());


            //Getting regions ids(keys) of all available regions from get_all_cities()
            let all_available_regions_keys: HashSet<i32> = get_all_cities().keys().cloned().collect();
            let mut current_alerts = Vec::<CurrentAlarm>::new();

            debug!("{} - creating task for update data", chrono::Local::now());

            let updated_date_time_clone = Arc::clone(&updated_date_time);
            let update_data_task = task::spawn(async move {

            let mut updated_date_time_lock = updated_date_time_clone.lock().await;


                /*
                    If there was an update of information on the server,data on current alarms is received. 
                    The data is placed in the variables current_alerts and updated_date_time_lock,
                    where current_alert is a vector of CurrentAlarm structures, 
                    and updated_at is the time of the last data change on the server.
                */
                let data = deserialize_current_alarms_data(updated_date_time_lock.clone().to_string()).await.unwrap();
                debug!("{} - getting for new data \t{}\n{:?}", chrono::Local::now(), data.1, data.0);

                debug!("{} - updating date_time from new value - {}", chrono::Local::now(), data.1);
                *updated_date_time_lock = data.1.clone(); 

                debug!("{} - getting current alerts - {:?}", chrono::Local::now(), data.0);
                current_alerts = data.0.clone();


                //Getting region IDs where the alarm is active
                let _current_alarm_location_ids: HashSet<i32> = {

                    let mut result = HashSet::<i32>::new();
                    for _v in current_alerts {
                        result.insert(_v._location_oblast_uid.into());
                    }
                    result

                };
                debug!("{} - regions with active alerts - {:?}", chrono::Local::now(), _current_alarm_location_ids);
                
                //Getting region IDs where the alarm is not active
                let _incactive_alarm_location_ids: HashSet<_> = all_available_regions_keys
                    .difference(&_current_alarm_location_ids)
                    .cloned()
                    .collect();
                debug!("{} - regions with inactive alerts - {:?}", chrono::Local::now(), _incactive_alarm_location_ids);
                

                //Update data using tasks
                for _v in _current_alarm_location_ids {
                    match task::spawn(
                        set_alarm_status(_v, true)
                    ).await {
                        Ok(_) => {
                            debug!("{:?} - current alarm in region {} updated", chrono::Local::now(), _v);
                        },
                        Err(e) => info!("Error occurred: {:?}", e),
                    }
                }
                for _z in _incactive_alarm_location_ids {
                    match task::spawn(
                        set_alarm_status(_z, false)
                    ).await {
                        Ok(_) => {
                            debug!("{:?} - current alarm in region {} updated", chrono::Local::now(), _z);
                        },
                        Err(e) => info!("Error occurred: {:?}", e),
                    }
                }

            });

            match update_data_task.await {
                Ok(_) => {

                    debug!("{} - data update was succesfully", chrono::Local::now());

                },
                Err(e) => {

                    debug!("{} - data was not updated - {:?}", chrono::Local::now(), e);

                }
            }

        }

    }


}