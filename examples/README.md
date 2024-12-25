# Examples

Here are some basic usages for the library.

## Talker

```bash
# Run bridge
zenoh-bridge-ros2dds
# Zenoh
./talker
# ROS
ros2 run demo_nodes_cpp listener
```

## Listener

```bash
# Run bridge
zenoh-bridge-ros2dds
# Zenoh
./listener
# ROS
ros2 run demo_nodes_cpp talker
```

## Service client

```bash
# Run bridge
zenoh-bridge-ros2dds
# Zenoh
./service_client
# ROS (Need to switch to CycloneDDS or it can't work)
export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
ros2 run demo_nodes_cpp add_two_ints_server
```
