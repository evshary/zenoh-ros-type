use std::time::Duration;

use cdr::{CdrLe, Infinite};
use zenoh::{Config, Wait};
use zenoh_ros_type::common_interfaces::std_msgs;

fn main() {
    let key_expr = "talker";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let publisher = session.declare_publisher(key_expr).wait().unwrap();
    let mut cnt = 0;
    loop {
        let data = format!("Hello World {}!", cnt);
        println!("Publish: {}", data);
        let msg = std_msgs::String_ { data };
        let payload = cdr::serialize::<_, _, CdrLe>(&msg, Infinite).unwrap();
        publisher.put(payload).wait().unwrap();
        std::thread::sleep(Duration::from_secs(1));
        cnt += 1;
    }
}
