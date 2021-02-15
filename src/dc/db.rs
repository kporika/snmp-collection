use postgres::{Client, NoTls};
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::Type;
use actix::prelude::*;
use crate::dc::udpserver::{Attribute,ValuePair, UdpSender, MessageCount} ;
use std::time::SystemTime;
// use tokio::time::{delay_for, Duration};
use slog::info ;


pub struct DbServer {
    pub client: Client ,
    pub senders: Vec<Addr<UdpSender>>,
    pub data: Vec<ValuePair>,
    pub count: usize,
    pub logger: slog::Logger,
}

impl Actor for DbServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        
        ctx.set_mailbox_capacity(200000);
        // add tables if not present
       /* let query:&str = "SELECT EXISTS(SELECT FROM information_schema.tables WHERE  table_schema ='public' AND table_name = 'polleddata')" ;
        let rows = self.client.query(query, &[]).unwrap();
        info!(self.logger, "checking the exsitence of DB Table, POLLEDDATA");
        let table_exists = rows[0].get::<_,bool>(0) ;
        if !table_exists {
            info!(self.logger, "POLLEDDATA Table does not exist");

            self.client.batch_execute("
                CREATE TABLE IF NOT EXISTS POLLEDDATA (
                    id INT PRIMARY KEY,
                    oid VARCHAR (50)  NOT NULL,
                    ip_addr VARCHAR (50) NOT NULL,
                    community VARCHAR (25)  NOT NULL 
                )"
            ).unwrap();
            info!(self.logger, "Created Polleddata");
            self.client.batch_execute("
                CREATE TABLE IF NOT EXISTS STATSDATA (
                    id INT,
                    timestamp TIMESTAMP,
                    vtype INT,
                    value Double Precision
                )"
            ).unwrap();
            info!(self.logger, "Created statsdata");

            for i in 1..13 {
                self.client.execute(
                    "INSERT INTO polleddata (id,oid,ip_addr,community) VALUES ($1, $2, $3, $4)",
                    &[&(100000+i), &format!("1.3.6.1.2.1.2.2.1.10.{}", i).as_str()  , &"192.168.1.1:161", &"public"],
                ).unwrap() ;
            }
            for i in 1..13 {
                self.client.execute(
                    "INSERT INTO polleddata (id,oid,ip_addr,community) VALUES ($1, $2, $3, $4)",
                    &[&(100012+i), &format!("1.3.6.1.2.1.2.2.1.16.{}", i).as_str() , &"192.168.1.1:161", &"public"],
                ).unwrap() ;
            }
            info!(self.logger, "inserted oids for sample snmp data collection");

        }
        */
        info!(self.logger, "Initializatin complete for DB Server Actor");
    }
    
}
impl DbServer {
    pub fn new(dburl:&str, senders: Vec<Addr<UdpSender>>, count:usize, logger: slog::Logger ) -> Self {
        Self {
            senders,
            client: Client::connect(dburl, NoTls).unwrap() ,
            data: vec![],
            count,
            logger
        }
    }
    fn dump_data(&mut self) {
        let sink = self.client.copy_in("COPY statsdata (id, timestamp, vtype,  value) FROM STDIN BINARY").unwrap();
        let mut writer = BinaryCopyInWriter::new(sink, &[Type::INT4, Type::TIMESTAMP, Type::INT4, Type::FLOAT8]);
        for vp in self.data.iter() {
            writer        
            .write(&[&vp.id,&vp.timestamp,&vp.vtype,&vp.value]).unwrap();
        }
        writer.finish().unwrap();
    }
    fn send_attr_for_collection(&mut self, query:&str) {
        info!(self.logger, "{:#?}", query);
        let rows = self.client.query(query, &[]).unwrap();
       
        for (i,row) in rows.iter().enumerate() {
            let _agent_id = row.get::<_, i32>(5) as usize ;
            
            self.senders.get(i%self.count).unwrap().do_send(
            // self.senders[i%self.count].do_send(
                Attribute::new(
                    row.get::<_, i32>(0),
                    row.get::<_, &str>(1),
                    row.get::<_, &str>(2),
                    row.get::<_, i32>(3),
                    row.get::<_, &str>(4)
                )
            );  
          //  delay_for(Duration::from_millis(2)) ;
        }
        for i in 0..self.count{
            self.senders.get(i).unwrap().do_send(MessageCount)
        }
    }
}


#[rtype(result = "()")]
#[derive(Debug, Message)]
pub struct StartDataCollection;
impl Handler<StartDataCollection> for DbServer {
    type Result =  ();
    fn handle(&mut self, _msg: StartDataCollection, _: &mut Context<Self>) {
        self.send_attr_for_collection("SELECT * FROM POLLEDDATA ORDER BY ID, AID");
    }   
}

#[rtype(result = "()")]
#[derive(Debug, Message)]
pub struct RepeatDataCollection{
    pub ts: SystemTime
}
impl Handler<RepeatDataCollection> for DbServer {
    type Result =  ();
    fn handle(&mut self, msg: RepeatDataCollection, _: &mut Context<Self>) {
        let ts = msg.ts.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.send_attr_for_collection(
            format!("select * from polleddata where id not in (select distinct(id) from statsdata where EXTRACT(epoch FROM timestamp) > {}) ORDER BY ID, AID", ts).as_str() 
        );
    }   
}

impl Handler<ValuePair> for DbServer {
    type Result =  ();
    fn handle(&mut self, msg: ValuePair, _: &mut Context<Self>) {
        self.data.push(msg) ;
    }
}

#[rtype(result = "()")]
#[derive(Debug, Message)]
pub struct BulkCopy;
impl Handler<BulkCopy> for DbServer {
    type Result =  ();
    fn handle(&mut self, _msg: BulkCopy, _: &mut Context<Self>) {
        if self.data.len() > 0 {
            self.dump_data();
            self.data.clear() ;
        }
    }
}
