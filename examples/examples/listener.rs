use zenoh::{Config, Wait};
use zenoh_ros_type::common_interfaces::std_msgs;

fn main() {
    let key_expr = "chatter";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let _subscriber = session
        .declare_subscriber(key_expr)
        .callback(|sample| {
            let msg: std_msgs::String_ = sample.payload().into();
            println!("Receive: {}", msg.data);
        })
        .wait()
        .unwrap();
    std::thread::park();
}
