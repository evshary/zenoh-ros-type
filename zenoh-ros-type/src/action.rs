/// The struct is used by ROS action.
/// If you want to sent ROS action with Zenoh directly. You should include the header.
/// Refer to https://design.ros2.org/articles/actions.html for more detail.
use serde_derive::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use zenoh::{bytes::ZBytes, sample::Sample, Config, Wait};
use zenoh_ros_derive::ZBytesCdr;

use crate::{builtin_interfaces::Time, rcl_interfaces::action_msgs, unique_identifier_msgs::UUID};

/// The response struct for Action SendGoal
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionSendGoalResponse {
    pub accept: bool, // Accept the request or not
    pub timestamp: Time,
}

/// The request struct for getting ActionResult
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionResultRequest {
    pub goal_id: UUID,
}

fn get_current_time() -> Time {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Time {
        sec: now.as_secs() as i32,
        nanosec: now.subsec_nanos() as u32,
    }
}

// Used by action client
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

    pub fn send_goal<T: Into<ZBytes>>(&self, req: T) -> ActionSendGoalResponse {
        let recv_handler = self.send_goal_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().into()
    }

    pub fn cancel_goal(&self, uuid: UUID) -> action_msgs::CancelGoalResponse {
        let req = action_msgs::CancelGoalRequest {
            goal_info: action_msgs::GoalInfo {
                goal_id: uuid,
                stamp: get_current_time(),
            },
        };
        let recv_handler = self.cancel_goal_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().into()
    }

    pub fn get_result(&self, uuid: UUID) -> ZBytes {
        let req = ActionResultRequest { goal_id: uuid };
        let recv_handler = self.get_result_client.get().payload(req).wait().unwrap();
        let reply_sample = recv_handler.recv().unwrap();
        reply_sample.result().unwrap().payload().clone()
    }
}
