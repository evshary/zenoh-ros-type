use zenoh::{Config, Wait};
use zenoh_ros_type::example_interfaces::srv;

fn main() {
    let a = 1;
    let b = 2;
    let key_expr = "add_two_ints";
    let session = zenoh::open(Config::default()).wait().unwrap();
    let client = session.declare_querier(key_expr).wait().unwrap();

    // Send the request
    println!("Send AddTwoIntsRequest: a={a}, b={b}");
    let req = srv::AddTwoIntsRequest { a, b };
    let recv_handler = client.get().payload(req).wait().unwrap();

    // Parse the reply
    let reply_sample = recv_handler.recv().unwrap();
    let reply: srv::AddTwoIntsReply = reply_sample.result().unwrap().payload().into();
    println!("Get result: sum={}", reply.sum);
}
