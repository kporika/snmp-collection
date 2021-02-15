use actix::prelude::*;
use std::net::SocketAddr;
use std::io::{Result};
use tokio_util::udp::UdpFramed ;
use bytes::BytesMut;
use tokio_util::codec::BytesCodec;
use futures::{StreamExt};
use actix::io::SinkWrite; 
use std::time::SystemTime;
use futures::stream::SplitStream ;
use tokio::time::{delay_for, Duration};
// use sled;
use slog;
use slog::Drain;
use slog::{info, o};
use slog_async;
use slog_term;
// use std::str::FromStr;
//use std::fs::File;
//use std::io;
//use std::sync::Arc;
//use futures::future::Future;
//use futures::future::TryFutureExt;


mod dc;
mod settings;
mod logger;

use settings::Settings;
use logger::ThreadLocalDrain;
use dc::udpserver::{UdpServer, UdpSender, UdpPacket} ;
use dc::db::{DbServer, StartDataCollection, RepeatDataCollection, BulkCopy} ;

// include!(concat!(env!("OUT_DIR"), "/version.rs"));


#[actix_rt::main]
async fn main() {
    
    let settings = Settings::new().unwrap(); 
    
    let mut senders =vec![] ;
    let mut servers = vec![] ;

    let logger = setup_slog(settings.logger.file_path.as_str(), settings.logger.log_level.as_str()) ;

    let dc = settings.datacollection ;
    for i in 0..dc.dc_actor_pairs {
        let(sender, stream ) = get_sender(dc.udp_bind_port + i as u32, logger.new(o!("thread_name"=>"PDU Sender"))).await ;
        senders.push(sender);
        servers.push(stream) ;
    }
    
    let sd = settings.database ;
    let db = DbServer::new(
        format!("host={} user={} password={} dbname={}", sd.host,sd.user, sd.password, sd.dbname ).as_str(), 
        senders, 
        dc.dc_actor_pairs ,
        logger.new(o!("thread_name"=>"DB Server"))
    ).start() ;
    
    for stream  in servers {
        start_servers(stream, db.clone(), logger.new(o!("thread_name"=>"UDP Reciever")) ).await ;
    }
    
    // let logger = logger.filter_level(slog::Level::from_str(settings.logger.log_level.as_str()).unwrap());
    info!(logger, "All Actors initialized ready for Data Collection");
    
    let mut interval_timer = tokio::time::interval(tokio::time::Duration::new(dc.interval_seconds,0));
    loop {
        interval_timer.tick().await;
        actix_rt::spawn(collect(db.clone(), dc.wait_time_millis, dc.retries)) ;
    }
    
   // actix_rt::Arbiter::local_join().await;
}

async fn get_sender(port:u32, logger: slog::Logger) -> (Addr<UdpSender>, SplitStream<UdpFramed<BytesCodec>> ){
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let sock = tokio::net::UdpSocket::bind(&addr).await.unwrap();
    let (sink, stream) = UdpFramed::new(sock, BytesCodec::new()).split();
    let dc = UdpSender::create(|ctx|  {
               UdpSender{
                    sender: SinkWrite::new(sink, ctx) ,
                    logger: logger.clone(),
                    count: 0
                }
     });

     (dc,stream)

}
async fn start_servers(stream:SplitStream<UdpFramed<BytesCodec>>, db:Addr<DbServer>, logger: slog::Logger) {
    let _server = UdpServer::create(|ctx| {
        ctx.add_stream(stream.filter_map(
            |item: Result<(BytesMut, SocketAddr)>| async {
                item.map(|(data, target)| UdpPacket{data, target}).ok()
            },
        ));

        UdpServer {
            server: db,  
            logger: logger.clone()
        }
    });
}

async fn collect(dc: Addr<DbServer>, wait_time_millis:u64, retries: u8) {
    let ts = SystemTime::now() ;
    dc.do_send(StartDataCollection) ;
    delay_for(Duration::from_millis(wait_time_millis)).await;
    dc.do_send(BulkCopy);
    for _ in 1..retries{
        dc.do_send(RepeatDataCollection{ts:ts}) ;
        delay_for(Duration::from_millis(wait_time_millis)).await;
        dc.do_send(BulkCopy);
    }
   
}
fn setup_slog(path:&str, _level:&str ) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(decorator).build().fuse() ;
    // json log file
    // "/var/tmp/actix-test.log"
    let logfile = std::fs::File::create(path).unwrap();
    let json_drain = slog_json::Json::new(logfile)
        .add_default_keys()
        // include source code location
        .add_key_value(o!("place" =>
           slog::FnValue(move |info| {
               format!("{}::({}:{})",
                       info.module(),
                       info.file(),
                       info.line(),
                )}),
                "sha"=>env!("VERGEN_SHA_SHORT")))
        .build()
        .fuse();    
    let dup_drain = slog::Duplicate::new(json_drain, term_drain);
    let async_drain = slog_async::Async::new(dup_drain.fuse()).build();
    let log = slog::Logger::root(ThreadLocalDrain { drain: async_drain }.fuse(), o!());
   // let log = log.filter_level(slog::Level::from_str(level).unwrap());
    log 
}