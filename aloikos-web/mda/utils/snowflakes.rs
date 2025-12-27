use snowflake::SnowflakeIdGenerator;
use crate::config::CONFIG;
pub fn new_id() -> i64 {
    let mut generator = SnowflakeIdGenerator::new(
        CONFIG.server_id as i32,
        CONFIG.server_datacenter_id as i32
    );
    generator.generate()
}