use postgres::{Client, NoTls};
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::Type;
use actix::prelude::*;
use crate::dc::udpserver::{Attribute,ValuePair, UdpSender} ;
use std::time::SystemTime;
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
       ctx.set_mailbox_capacity(10000);
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
            self.senders[i%self.count].do_send(
                Attribute::new(
                    row.get::<_, i32>(0),
                    row.get::<_, &str>(1),
                    row.get::<_, &str>(2),
                    row.get::<_, &str>(3)
                )
            );  
           
        }
    }
}


#[rtype(result = "()")]
#[derive(Debug, Message)]
pub struct StartDataCollection;
impl Handler<StartDataCollection> for DbServer {
    type Result =  ();
    fn handle(&mut self, _msg: StartDataCollection, _: &mut Context<Self>) {
        self.send_attr_for_collection("SELECT * FROM POLLEDDATA");
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
            format!("select * from polleddata where id not in (select distinct(id) from statsdata where EXTRACT(epoch FROM timestamp) > {})", ts).as_str() 
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
