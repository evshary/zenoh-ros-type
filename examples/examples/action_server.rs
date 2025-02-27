use std::{sync::mpsc, time::Duration};

use zenoh::{Config, Wait};
use zenoh_ros_type::{
    action, builtin_interfaces, example_interfaces::action as example_action,
    rcl_interfaces::action_msgs, unique_identifier_msgs::UUID,
};

fn main() {
    let key_expr = "fibonacci";
    let send_goal_expr = key_expr.to_string() + "/_action/send_goal";
    let cancel_goal_expr = key_expr.to_string() + "/_action/cancel_goal";
    let get_result_expr = key_expr.to_string() + "/_action/get_result";
    let feedback_expr = key_expr.to_string() + "/_action/feedback";
    let status_expr = key_expr.to_string() + "/_action/status";

    let (tx, rx) = mpsc::channel();

    // Declare the publisher / queryable for the action server
    let session = zenoh::open(Config::default()).wait().unwrap();
    let feedback_publisher = session.declare_publisher(feedback_expr).wait().unwrap();
    let status_publisher = session.declare_publisher(status_expr).wait().unwrap();
    let _send_goal_server = session
        .declare_queryable(send_goal_expr.clone())
        .callback(move |query| {
            let send_goal: example_action::FibonacciSendGoal = query.payload().unwrap().into();
            println!("Receive {:?}: {:?}", send_goal.goal_id, send_goal.goal);
            tx.send(send_goal).unwrap();

            // Reply to the action client
            let send_goal_response = action::ActionSendGoalResponse {
                accept: true,
                // TODO: We should have a correct timestamp
                timestamp: builtin_interfaces::Time { sec: 0, nanosec: 0 },
            };
            query
                .reply(&send_goal_expr, send_goal_response)
                .wait()
                .unwrap();
        })
        .wait()
        .unwrap();
    let get_result_server = session
        .declare_queryable(get_result_expr.clone())
        .wait()
        .unwrap();
    let _cancel_goal_server = session
        .declare_queryable(cancel_goal_expr)
        .callback(|query| {
            println!("{:?}", query);
        })
        .wait()
        .unwrap();

    loop {
        let send_goal = rx.recv().unwrap();
        let goal_id = send_goal.goal_id;
        let goal = send_goal.goal;

        // Publish status EXECUTING
        let msg = action_msgs::GoalStatusArray {
            status_list: vec![action_msgs::GoalStatus {
                goal_info: action_msgs::GoalInfo {
                    goal_id: UUID { uuid: goal_id },
                    // TODO: We should have a correct timestamp
                    stamp: builtin_interfaces::Time { sec: 0, nanosec: 0 },
                },
                status: action_msgs::goal_status::EXECUTING,
            }],
        };
        println!(
            "Publish status {:?}: {:?}",
            msg.status_list[0].goal_info.goal_id, msg.status_list[0].status
        );
        status_publisher.put(msg).wait().unwrap();

        // Calculate and publish the feedback
        let mut feedback = vec![0, 1];
        // give me a for loop with i increasing until 10
        for _ in 1..goal {
            feedback.push(feedback[feedback.len() - 1] + feedback[feedback.len() - 2]);
            println!("Publish feedback: {:?}", feedback);
            let msg = example_action::FibonacciFeedback {
                goal_id,
                sequence: feedback.clone(),
            };
            feedback_publisher.put(msg).wait().unwrap();
            std::thread::sleep(Duration::from_secs(1));
        }

        // Publish status SUCCEEDED
        let msg = action_msgs::GoalStatusArray {
            status_list: vec![action_msgs::GoalStatus {
                goal_info: action_msgs::GoalInfo {
                    goal_id: UUID { uuid: goal_id },
                    // TODO: We should have a correct timestamp
                    stamp: builtin_interfaces::Time { sec: 0, nanosec: 0 },
                },
                status: action_msgs::goal_status::SUCCEEDED,
            }],
        };
        println!(
            "Publish status {:?}: {:?}",
            msg.status_list[0].goal_info.goal_id, msg.status_list[0].status
        );
        status_publisher.put(msg).wait().unwrap();

        // Reply the get_result
        let query = get_result_server.recv().unwrap();
        let get_result: action::ActionResultRequest = query.payload().unwrap().into();
        println!("Get result goal_id: {:?}", get_result.goal_id);
        let get_result_response = example_action::FibonacciResult {
            status: action_msgs::goal_status::SUCCEEDED,
            sequence: feedback,
        };
        query
            .reply(&get_result_expr, get_result_response)
            .wait()
            .unwrap();
    }
}
