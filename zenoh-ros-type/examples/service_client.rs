use cdr::{CdrLe, Infinite};
use zenoh::{Config, Wait};
use zenoh_ros_type::example_interfaces::srv;

fn main() {
    let a = 1;
    let b = 2;
    let key_expr = "add_two_ints";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let client = session.declare_querier(key_expr).wait().unwrap();

    // Send the request
    println!("Send AddTwoIntsRequest: a={}, b={}", a, b);
    let req = srv::AddTwoIntsRequest { a, b };
    let buf = cdr::serialize::<_, _, CdrLe>(&req, Infinite).unwrap();
    let recv_handler = client.get().payload(buf).wait().unwrap();

    // Parse the reply
    let reply_sample = recv_handler.recv().unwrap();
    let reader = reply_sample.result().unwrap().payload().reader();
    let reply: srv::AddTwoIntsReply = cdr::deserialize_from(reader, cdr::size::Infinite).unwrap();
    println!("Get result: sum={}", reply.sum);
}
