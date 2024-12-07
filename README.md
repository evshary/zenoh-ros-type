# zenoh-ros-type

Common Rust struct for ROS 2 messages used by Zenoh.

We can communicate to the ROS application behind [zenoh-bridge-dds](https://github.com/eclipse-zenoh/zenoh-plugin-dds) and [zenoh-bridge-ros2dds](https://github.com/eclipse-zenoh/zenoh-plugin-ros2dds) with Zenoh Rust API.

* [API documentation](https://docs.rs/zenoh-ros-type)
* [crates.io](https://crates.io/crates/zenoh-ros-type)

## Example

There are some examples for how to use zenoh-ros-type in your application.

* [autoware_manual_control_rs](https://github.com/evshary/autoware_manual_control_rs): Control multiple Autoware with the help of Zenoh.
* [zenoh_carla_bridge](https://github.com/evshary/zenoh_carla_bridge): Bridge the Carla and Autoware with Zenoh.

## For developers

* Install pre-commit hook

```shell
pre-commit install --install-hooks
```
