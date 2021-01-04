# Snmp Data Collection using Rust and Actix
Snmp Data Collection is the process of collecting performance data from various agents in the network using Snmp. In this method instead of focussing on one agent at a time to do the data collection, we cull all the attributes that need to be polled from all agents into a single DB table called POLLEDDATA. Here, all the tabuar data, i.e. data of type that has multiple indexes will get converted into a unique instance. For example, ifOctets attribute for an agent may have multiple indexes, say 100. in this method we will have 100 unique attributes to be polled with ifOCtets.index for all the indexes for that agent. Similarly all scalar attributes will be padded with .0. POLLEDDATA table to contain all the data, but may have bit of duplication in the fields of agent, version and community. see the sample of a POLLEDDATA table

  ```sh
CREATE TABLE POLLEDDATA (
	id INT PRIMARY KEY,
	oid VARCHAR (50)  NOT NULL,
	ip_addr VARCHAR (50) NOT NULL,
	community VARCHAR (25)  NOT NULL
);

INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100001, '1.3.6.1.2.1.2.2.1.10.1', '192.168.1.1:161',  'public') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100002, '1.3.6.1.2.1.2.2.1.10.2', '192.168.1.1:161', 'public') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100003, '1.3.6.1.2.1.2.2.1.10.3', '192.168.1.1:161', 'public') ;
```

After defining the polled data, we need a mechanism to schedule the data collection using the cron job or threads. In this case, i'm using actix (Highly scalable Actor framework in rust ) to manage the state of the actors and the data collection.

# Here, we have 3 main Actors(threads) for Data Collection.
  - Snmp Sender 
  - Snmp Receiver
  - DB Server 
  
Here Snmp Sender/Receiver are a pair of  actors spawn on the same UDPSocket. This actor pair(s) is configurable from the config file.

DB Server is a single actor that reads the POLLEDDATA from the DB (postgres) and sends the attributes to Snmp Senders for Data collection.
Snmp Receivers receive the PDU packets and decode the packets and send the value to the DB Server. 
DBServer uses Postgres Bulk Copy to insert the Data into Postgres DB.
See the structure of the Collected Data 
```sh
CREATE TABLE STATSDATA (
	id INT,
	timestamp TIMESTAMP,
	vtype INT,
	value Double Precision
);
```
### Column definitions
    - ID is the id of the attribute from the POLLEDDATA table
    - timestamp 
    - vtype  (ValueType so that overflows can be caliberated at DB level)
    - value (actual value of the data converted to double )

When the first round of data collection is done,  2nd and 3rd rounds are done by selecting IDs that did not get the value back from the previous rounds for that datacollection period.

```sh
select * from polleddata where id not in (select distinct(id) from statsdata where EXTRACT(epoch FROM timestamp) > ts_at_the_beginning_of_data_collection )
```


To compile the program, Install Rust related dependencies 
```sh
cargo build --release
```
To run
```sh
cargo run --release
```
All the configurations/knobs for the application can be changed from the corresponding .toml files under config directory.

Sample Configuration

```sh
debug=true
#postgres database settings
[database]
dbname="dc"
user="test"
password="ciena123"
host="localhost"

[datacollection]
# number of collection pairs, udp sockets for data collection, each pair consists of sender and recievr
dc_actor_pairs=24
# retries, 
retries=3
# wait time between retries ,  retries*wait_time_millis should be less than interval_seconds 
wait_time_millis=100000
#periodicty of data collection in seconds
interval_seconds=300
# starting udp port for binding to a udpsocket, we will be binding dc_actor_pairs of udpscokets starting with the below port
udp_bind_port=8161

[logger]
# log location of the log file_path
file_path="/var/tmp/actix-test.log"
# filtering level , above this level will be logged, 
# available options are Off,Critical,Error,Warning,Info,Debug,Trace,
log_level="Info"
```

