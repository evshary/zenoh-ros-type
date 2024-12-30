use zenoh::{Config, Wait};
use zenoh_ros_type::example_interfaces::srv;

fn main() {
    let key_expr = "add_two_ints";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let _queryable = session
        .declare_queryable(key_expr)
        .callback(move |query| {
            let request: srv::AddTwoIntsRequest = query.payload().unwrap().into();
            println!("Receive a={}, b={}", request.a, request.b);
            let response = srv::AddTwoIntsReply {
                sum: request.a + request.b,
            };
            println!("Send back {}", response.sum);
            query.reply(key_expr, response).wait().unwrap();
        })
        .wait()
        .unwrap();
    std::thread::park();
}
