#![forbid(unsafe_code)]
use std::ops;
use sabaton_mw::{AsyncReader, SubscribeOptions};
use std::{time::Duration, sync::Arc};
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError, SyncReader, Samples};
use tracing::{debug, info, span, Level};

use robotics_signals::{
    sensors::image::{Encoding, Image1080p4BPP},
    standard::Header,
    standard::Timestamp,
};

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
    .with_shared_memory(true)
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");
        let mut shm_subscribe_options = SubscribeOptions::default();

    
    let mut reader= node.subscribe_async::<Image1080p4BPP>(&shm_subscribe_options).expect("Unable to advertise");
    
    let res = node.spin(move || {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {
            //let mut vehicle_safe:bool;

            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
                
                //let mut CurrentIgnition:bool;
                let mut images = Samples::<Image1080p4BPP>::new(1);
                if let Ok(res) = reader.take(&mut images).await{
                    for image in images.iter(){
                    println!("WIDTH: {}",image.width);
                    println!("HEADER: {}",image.header.seq);
                    println!("HEADER: {}",image.header.stamp.sec);
                }}
                
                

            }

         });
         
    });
    res

}