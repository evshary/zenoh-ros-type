use cdr::{CdrLe, Infinite};
use zenoh::{Config, Wait};
use zenoh_ros_type::example_interfaces::srv;

fn main() {
    let key_expr = "add_two_ints";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let _queryable = session
        .declare_queryable(key_expr)
        .callback(move |query| {
            let request: srv::AddTwoIntsRequest =
                cdr::deserialize(&query.payload().unwrap().to_bytes()).unwrap();
            println!("Receive a={}, b={}", request.a, request.b);
            let response = srv::AddTwoIntsReply {
                sum: request.a + request.b,
            };
            println!("Send back {}", response.sum);
            let data = cdr::serialize::<_, _, CdrLe>(&response, Infinite).unwrap();
            query.reply(key_expr, data).wait().unwrap();
        })
        .wait()
        .unwrap();
    std::thread::park();
}
