# zenoh-ros-type

Common Rust struct for ROS 2 messages used by Zenoh.

We can communicate to the ROS application behind [zenoh-bridge-ros2dds](https://github.com/eclipse-zenoh/zenoh-plugin-ros2dds) with Zenoh Rust API.

* [API documentation](https://docs.rs/zenoh-ros-type)
* [crates.io](https://crates.io/crates/zenoh-ros-type)

The library supports the following message type:

* [common_interface](https://github.com/ros2/common_interfaces): Common-used ROS message
* [example_interfaces](https://github.com/ros2/example_interfaces): Example interfaces in ROS 2
* [rcl_interface](https://github.com/ros2/rcl_interfaces): Common interface in RCL
* [unique_identifier_msgs](https://github.com/ros2/unique_identifier_msgs): UUID message type in ROS 2
* [autoware_auto_msgs](https://github.com/tier4/autoware_auto_msgs/tree/tier4/main): Messages used in Autoware
* [tier4_autoware_msgs](https://github.com/tier4/tier4_autoware_msgs/tree/tier4/universe): Messages used in Autoware

Not all of them have been added. Feel free to contribute it.

## Examples

You can check the examples folder for the basic usage.

Also, there are some examples for how to use zenoh-ros-type in your application.

* [autoware_manual_control_rs](https://github.com/evshary/autoware_manual_control_rs): Control multiple Autoware with the help of Zenoh.
* [zenoh_carla_bridge](https://github.com/evshary/zenoh_carla_bridge): Bridge the Carla and Autoware with Zenoh.

## For developers

* Install pre-commit hook

```shell
pre-commit install --install-hooks
```
