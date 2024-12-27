use cdr::{CdrLe, Infinite};
use zenoh::{Config, Wait};
use zenoh_ros_type::{action, example_interfaces::action as example_action};

fn main() {
    let key_expr = "fibonacci";
    let send_goal_expr = key_expr.to_string() + "/_action/send_goal";
    let cancel_goal_expr = key_expr.to_string() + "/_action/cancel_goal";
    let get_result_expr = key_expr.to_string() + "/_action/get_result";
    let feedback_expr = key_expr.to_string() + "/_action/feedback";
    let status_expr = key_expr.to_string() + "/_action/status";

    // Declare the querier / subscriber for the action client
    let session = zenoh::open(Config::default()).wait().unwrap();
    let send_goal_client = session.declare_querier(send_goal_expr).wait().unwrap();
    let get_result_client = session.declare_querier(get_result_expr).wait().unwrap();
    let _cancel_goal_client = session.declare_querier(cancel_goal_expr).wait().unwrap();
    let _feedback_subscriber = session
        .declare_subscriber(feedback_expr)
        .callback(|sample| {
            let feedback: example_action::FibonacciFeedback =
                cdr::deserialize_from(sample.payload().reader(), cdr::size::Infinite).unwrap();
            println!(
                "The feedback of {:?}: {:?}",
                feedback.goal_id, feedback.sequence
            );
        })
        .wait()
        .unwrap();
    let _status_subscriber = session
        .declare_subscriber(status_expr)
        .callback(|sample| {
            let status: action::ActionStatus =
                cdr::deserialize_from(sample.payload().reader(), cdr::size::Infinite).unwrap();
            println!("The status of {:?}: {:?}", status.goal_id, status.status);
        })
        .wait()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Send goal client
    let req = example_action::FibonacciSendGoal {
        goal_id: [1; 16], // TODO: We should use random here
        goal: 10,
    };
    let buf = cdr::serialize::<_, _, CdrLe>(&req, Infinite).unwrap();
    let recv_handler = send_goal_client.get().payload(buf).wait().unwrap();
    let reply_sample = recv_handler.recv().unwrap();
    let reader = reply_sample.result().unwrap().payload().reader();
    let reply: action::ActionSendGoalResponse =
        cdr::deserialize_from(reader, cdr::size::Infinite).unwrap();
    println!("The result of SendGoal: {:?}", reply.accept);

    //// Cancel goal client
    //std::thread::sleep(std::time::Duration::from_secs(1));
    //let req = action::ActionCancelRequest {
    //    goal_id: [1; 16],
    //    timestamp: zenoh_ros_type::builtin_interfaces::Time { sec: 0, nanosec: 0 },
    //};
    //let buf = cdr::serialize::<_, _, CdrLe>(&req, Infinite).unwrap();
    //let recv_handler = _cancel_goal_client.get().payload(buf).wait().unwrap();
    //let reply_sample = recv_handler.recv().unwrap();
    //let reader = reply_sample.result().unwrap().payload().reader();
    //let reply: action::ActionCancelResponse =
    //    cdr::deserialize_from(reader, cdr::size::Infinite).unwrap();
    //println!("Cancel {:?}: {:?}", reply.goal_id, reply.response_code);

    // Wait for the result
    std::thread::sleep(std::time::Duration::from_secs(10));

    // Get result client
    let req = action::ActionResultRequest { goal_id: [1; 16] };
    let buf = cdr::serialize::<_, _, CdrLe>(&req, Infinite).unwrap();
    let recv_handler = get_result_client.get().payload(buf).wait().unwrap();
    let reply_sample = recv_handler.recv().unwrap();
    let reader = reply_sample.result().unwrap().payload().reader();
    let reply: example_action::FibonacciResult =
        cdr::deserialize_from(reader, cdr::size::Infinite).unwrap();
    println!("The result: {:?} {:?}", reply.status, reply.sequence);
}
