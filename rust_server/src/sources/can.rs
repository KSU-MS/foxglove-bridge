mod can_2_fb;
mod socketcan_utils;

use crate::sources::{
    can::{can_2_fb::can2fb, socketcan_utils::test_joe},
    FoxgloveLoggerTopic,
};
use dbc_rs::Dbc;
use foxglove::{RawChannel, Schema};
use std::{collections::HashMap, sync::Arc};

struct CanLogger {
    // TODO: Maybe axe these?
    dbc_path: String,
    fbs_path: String,
    bfbs_path: String,

    dbc: Dbc,
    bfbs_data: Vec<u8>,

    // TODO: Get something for socket can in dis bihh
    message_map: HashMap<u32, Arc<RawChannel>>, // Maps CAN ID to FBS encoder
    channel_map: HashMap<String, Arc<RawChannel>>, // Maps FBS table to Foxglove Channel
}

impl CanLogger {
    pub fn new(dbc_path: String, fbs_path: String, bfbs_path: String) -> CanLogger {
        let bfbs_data = std::fs::read(&bfbs_path)
            .map_err(|e| format!("Failed to read BFBS: {e}"))
            .unwrap()
            .to_vec();

        let dbc = Dbc::from_file(&dbc_path)
            .map_err(|e| format!("Failed to read DBC: {e}"))
            .unwrap();

        CanLogger {
            dbc_path,
            fbs_path,
            bfbs_path,
            dbc,
            bfbs_data,
            message_map: HashMap::new(),
            channel_map: HashMap::new(),
        }
    }

    pub fn log_can_message(&self, can_id: u32, payload: Vec<u8>) {
        let message_name = self
            .dbc
            .messages()
            .find_by_id(can_id)
            .unwrap()
            .name()
            .to_ascii_lowercase();

        // Find fbs field in HashMap
        let channel = self.channel_map.get(&message_name);

        // Decode can message

        // Find thing in fbs/bfbs

        // encode into bfbs

        // foxglove log
        // channel.unwrap().log(msg);
    }
}

impl FoxgloveLoggerTopic for CanLogger {
    fn start_log(&self) -> Result<(), String> {
        return Ok(());
    }

    fn stop_log(&self) -> Result<(), String> {
        return Ok(());
    }

    fn register_messages(
        &mut self,
        foxglove_context: crate::foxglove_utils::FoxgloveRuntime,
    ) -> Result<(), String> {
        for signal in self.dbc.messages().iter() {
            let message_name = signal.name();

            let channel = foxglove_context
                .ctx
                .channel_builder(format!("{message_name}_data"))
                .schema(Schema {
                    name: "flatbuffer".to_string(),
                    encoding: "flatbuffer".to_string(),
                    data: std::borrow::Cow::Owned(self.bfbs_data.clone()),
                })
                .message_encoding("flatbuffer".to_string());

            self.channel_map.insert(
                message_name.to_string(),
                channel
                    .build_raw()
                    .map_err(|e| format!("Couldn't build foxglove channel: {e}"))
                    .unwrap(),
            );
        }

        return Ok(());
    }
}

pub fn test_can(foxglove_context: crate::foxglove_utils::FoxgloveRuntime) {
    let mut can_logger = CanLogger::new(
        std::env::var_os("DBC_PATH")
            .expect("DBC_PATH not set")
            .to_string_lossy()
            .to_string(),
        std::env::var_os("FBS_PATH")
            .expect("FBS_PATH not set")
            .to_string_lossy()
            .to_string(),
        std::env::var_os("BFBS_PATH")
            .expect("BFBS_PATH not set")
            .to_string_lossy()
            .to_string(),
    );

    let reg_result = can_logger.register_messages(foxglove_context);
    println!("Register Result: {:?}", reg_result);

    can2fb(can_logger.bfbs_data.clone());

    // let ex_buf = vec![235, 25, 176, 30, 235, 177, 215, 30];
    //
    // println!(
    //     "{:?}",
    //     can_logger.dbc.decode(0x0A3, &ex_buf, false).unwrap()
    // );
    // println!(
    //     "{:?}",
    //     can_logger.dbc.messages().find_by_id(0x0A3).unwrap().name()
    // );

    // test_joe(can_logger);

    // socketcan_utils::test_can(dbc).unwrap()
}
