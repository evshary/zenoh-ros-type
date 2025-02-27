use zenoh::{bytes::ZBytes, sample::Sample, Config, Wait};
use zenoh_ros_type::{
    action, example_interfaces::action as example_action, rcl_interfaces::action_msgs,
    unique_identifier_msgs::UUID,
};

// TODO: Move to action.rs
pub struct ZenohActionClient<'a> {
    key_expr: String,
    session: zenoh::session::Session,
    send_goal_client: zenoh::query::Querier<'a>,
    get_result_client: zenoh::query::Querier<'a>,
    cancel_goal_client: zenoh::query::Querier<'a>,
    feedback_subscriber: Option<zenoh::pubsub::Subscriber<()>>,
    status_subscriber: Option<zenoh::pubsub::Subscriber<()>>,
}

impl ZenohActionClient<'_> {
    pub fn new(key_expr: &str) -> Self {
        let send_goal_expr = key_expr.to_string() + "/_action/send_goal";
        let cancel_goal_expr = key_expr.to_string() + "/_action/cancel_goal";
        let get_result_expr = key_expr.to_string() + "/_action/get_result";

        // Declare the querier / subscriber for the action client
        let session = zenoh::open(Config::default()).wait().unwrap();
        let send_goal_client = session.declare_querier(send_goal_expr).wait().unwrap();
        let get_result_client = session.declare_querier(get_result_expr).wait().unwrap();
        let cancel_goal_client = session.declare_querier(cancel_goal_expr).wait().unwrap();

        ZenohActionClient {
            key_expr: key_expr.to_string(),
            session,
            send_goal_client,
            get_result_client,
            cancel_goal_client,
            feedback_subscriber: None,
            status_subscriber: None,
        }
    }

    pub fn feedback_callback<F>(&mut self, callback: F)
    where
        F: Fn(Sample) + Send + Sync + 'static,
    {
        let feedback_expr = self.key_expr.to_string() + "/_action/feedback";

        let feedback_subscriber = self
            .session
            .declare_subscriber(feedback_expr)
            .callback(callback)
            .wait()
            .unwrap();
        self.feedback_subscriber = Some(feedback_subscriber);
    }

    pub fn status_callback<F>(&mut self, callback: F)
    where
        F: Fn(Sample) + Send + Sync + 'static,
    {
        let status_expr = self.key_expr.to_string() + "/_action/status";

        let status_subscriber = self
            .session
            .declare_subscriber(status_expr)
            .callback(callback)
            .wait()
            .unwrap();
        self.status_subscriber = Some(status_subscriber);
    }

    pub fn send_goal<T: Into<ZBytes>>(&self, req: T) -> action::ActionSendGoalResponse {
        let recv_handler = self.send_goal_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().into()
    }

    pub fn cancel_goal(&self, uuid: UUID) -> action_msgs::CancelGoalResponse {
        let req = action_msgs::CancelGoalRequest {
            goal_info: action_msgs::GoalInfo {
                goal_id: uuid,
                // TODO: We should have a correct timestamp
                stamp: zenoh_ros_type::builtin_interfaces::Time { sec: 0, nanosec: 0 },
            },
        };
        let recv_handler = self.cancel_goal_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().into()
    }

    pub fn get_result(&self, uuid: UUID) -> ZBytes {
        let req = action::ActionResultRequest { goal_id: uuid };
        let recv_handler = self.get_result_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().clone()
    }
}

fn main() {
    let key_expr = "fibonacci";
    let mut action_client = ZenohActionClient::new(key_expr);

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
    let uuid = UUID { uuid: [1; 16] }; // TODO: We should use random here
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
