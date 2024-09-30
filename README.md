Hello!  
This API is designed to receive current data on air raid alarms in the regions of Ukraine. The data is updated every 7 seconds.
To receive the necessary data, you can use the following methods:


/get_regions - If you need a list of all available regions.
		
  	For example: 127.0.0.1:8000/get_regions
	
/get_alarm/<id> - Get information about the alarm status in the specified region. 
 	<id> - integer region identifier.

  	For example: 127.0.0.1:8000/get_alarm/9
  
/get_alarms?<params..> - Get information about the alarm status in the specified regions.

 	For example: 127.0.0.1:8000/get_alarms?location1=15&location2=18&location3=31
    
