use actix::prelude::*;
use std::net::SocketAddr;
use snmp::{pdu, SnmpPdu, Value} ;
use tokio_util::udp::UdpFramed ;
use bytes::BytesMut;
use tokio_util::codec::BytesCodec;
use futures::stream::SplitSink;
use bytes::Bytes;
use actix::{Actor, Context, StreamHandler};
use actix::io::SinkWrite; 
use std::time::SystemTime;
use super::db::DbServer ;
use slog::{info, debug};
pub type SinkItem = (Bytes, SocketAddr);
pub type UdpSink = SplitSink<UdpFramed<BytesCodec>, SinkItem>;


pub struct UdpServer {
    pub server: actix::Addr<DbServer>,
    pub logger: slog::Logger,
}
   
impl Actor for UdpServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        info!(self.logger, "UDP Server started");
       ctx.set_mailbox_capacity(10000);
    }
    fn stopping(&mut self, _: &mut Context<Self>) -> Running {
        System::current().stop();
        Running::Stop
    }
}
#[rtype(result = "()")]
#[derive(Debug, Message)]
pub struct UdpPacket{
   pub data:BytesMut, 
   pub target: SocketAddr
}

#[rtype(result = "()")]
#[derive(Debug,Clone, Message)]
pub struct ValuePair{
   pub timestamp: SystemTime,
   pub id: i32,
   pub vtype: i32,
   pub value: f64
}
impl ValuePair {
    pub fn new(id:i32, vtype:i32, value:f64) -> Self {
        Self {
            id,
            value,
            vtype,
            timestamp: SystemTime::now()
        }
    }
}

#[rtype(result = "()")]
#[derive(Debug,Clone, Message)]
pub struct Attribute {
   pub id: i32,
   pub oid: String,
   pub ip_addr: String,
   // port: u32,
   pub community: String,
}

impl  Attribute {
    pub fn new(id:i32, oid:&str ,ip_addr:&str,community:&str) -> Self {
        Self{
            id, 
            oid: oid.to_string(),
            ip_addr: ip_addr.to_string(),
            community: community.to_string()
        }
    }
}

impl StreamHandler<UdpPacket> for UdpServer{
    fn handle(&mut self, msg: UdpPacket, _: &mut Context<Self>) {
           // let mydata = &data.freeze() ;
           let mut resp = SnmpPdu::from_bytes(&msg.data).unwrap();
           debug!(self.logger, "{:#?}", resp);
           if let Some((_oid, value )) = resp.varbinds.next() {
            match value {
                Value::Counter32(val)  => self.server.do_send(ValuePair::new(resp.req_id, 64,  val as f64)),
                Value::Counter64(val)  => self.server.do_send(ValuePair::new(resp.req_id, 70, val as f64)),
                Value::Integer(val) => self.server.do_send(ValuePair::new(resp.req_id, 80, val as f64)),
                _ =>  self.server.do_send(ValuePair::new(resp.req_id, 0,  0f64))
            }
            
        }  
    }
}
pub struct UdpSender{
    pub sender: SinkWrite<SinkItem, UdpSink>,
    pub logger: slog::Logger,

}

impl  Actor for UdpSender {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) { 
        info!(self.logger, "UDP Sender started");
        ctx.set_mailbox_capacity(1000);
    }
    fn stopping(&mut self, _: &mut Context<Self>) -> Running {
        System::current().stop();
        Running::Stop
    }
}
impl actix::io::WriteHandler<std::io::Error> for UdpSender {}
impl Handler<Attribute> for UdpSender {
    type Result = ();
    fn handle(&mut self, attr: Attribute, _: &mut Context<Self>)  {
        debug!(self.logger, "{:#?}", attr);
        let mut buf = pdu::Buf::default() ;   
        pdu::build_get(attr.community.as_bytes(), attr.id, get_oid_array(attr.oid.as_str()).as_slice(), &mut buf) ;               
        self.sender.write((Bytes::from(buf[..].to_vec()) , attr.ip_addr.parse::<SocketAddr>().unwrap())); 
    }
}
fn get_oid_array(oid:&str) -> Vec<u32> { 
    oid.split('.').collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap_or(0)).collect()
}





