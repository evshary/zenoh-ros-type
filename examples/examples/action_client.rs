use rand::Rng;
use zenoh_ros_type::{
    action, example_interfaces::action as example_action, rcl_interfaces::action_msgs,
    unique_identifier_msgs::UUID,
};

fn main() {
    let key_expr = "fibonacci";
    let mut action_client = action::ZenohActionClient::new(key_expr);

    // feedback subscriber
    action_client.feedback_callback(|sample| {
        let feedback: example_action::FibonacciFeedback = sample.payload().into();
        println!(
            "The feedback of {:?}: {:?}",
            feedback.goal_id, feedback.sequence
        );
    });

    // status subscriber
    action_client.status_callback(|sample| {
        let status_array: action_msgs::GoalStatusArray = sample.payload().into();
        for status in status_array.status_list {
            println!(
                "The status of {:?}: {:?}",
                status.goal_info.goal_id, status.status
            );
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Send goal client
    let mut rng = rand::rng();
    let uuid = UUID {
        uuid: rng.random::<[u8; 16]>(),
    };
    let req = example_action::FibonacciSendGoal {
        goal_id: uuid.clone(),
        goal: 10,
    };
    let reply = action_client.send_goal(req);
    println!("The result of SendGoal: {:?}", reply.accept);

    //// Cancel goal client
    //std::thread::sleep(std::time::Duration::from_secs(1));
    //let reply = action_client.cancel_goal(uuid.clone());
    //println!(
    //    "Cancel {:?}: {:?}",
    //    reply.goals_canceling, reply.return_code
    //);

    // Wait for the result
    std::thread::sleep(std::time::Duration::from_secs(10));

    // Get result client
    let reply: example_action::FibonacciResult = action_client.get_result(uuid.clone()).into();
    println!("The result: {:?} {:?}", reply.status, reply.sequence);
}
