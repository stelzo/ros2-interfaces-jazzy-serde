# ROS2 Interfaces for Rust

> This crate is a updated version of [this crate](https://crates.io/crates/ros2-interfaces-jazzy), that does not have a linked repository to apply changes to. For supporting later versions of ros2-client, we need to keep the dependency updated. All the credit of generating the messages goes to the original authors.

This repository contains Rust structs for all interfaces (i.e., messages and services) that are listed as releases
on the [ROS Index for Jazzy](https://index.ros.org/packages/#jazzy).

The default `ros2-client` feature implements the `ros2_client::Message` trait for every message.
For transport-independent structs with Serde serialization, disable default features.
This avoids compiling `ros2-client` and its DDS runtime.

Every package is a separate feature for this crate, so you can cherry-pick the interfaces you need.
Example are `std_msgs`, `geometry_msgs` and `nav2_msgs`. 

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ros2-interfaces-jazzy-serde = { version = "*", features = ["std_msgs"] }  # replace with the latest version and features you need
```

For message types without a ROS client or DDS runtime:

```toml
[dependencies]
ros2-interfaces-jazzy-serde = { version = "0.0.7", default-features = false, features = ["sensor_msgs", "nav_msgs"] }
```

Enable `ros2-client` explicitly when connecting these types to a ROS DDS node.
The `all` feature selects all interface packages. It does not enable transport integration.
Struct fields and Serde serialization are identical in both modes.

Then you can use the interfaces in your code:

```rust
use ros2_client::{Context, MessageTypeName, Name, NodeName, NodeOptions, QosProfile};
use ros2_interfaces_jazzy_serde::std_msgs;

fn main() {
    let context = Context::new().unwrap();
    let mut node = context
        .new_node(
            NodeName::new("/", "talker").unwrap(),
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();

    let topic = node
        .create_topic(
            &Name::new("/", "chatter").unwrap(),
            MessageTypeName::new("std_msgs", "String"),
            &QosProfile::publisher_default(),
        )
        .unwrap();

    let publisher = node
        .create_publisher::<std_msgs::msg::String>(&topic, None)
        .unwrap();

    let message = std_msgs::msg::String {
        data: "Hello, world!".to_string(),
    };

    publisher.publish(message).unwrap();
}
```


## Known Issues

### Disabled Interfaces
The following packages (and thereby: features) do not compile due to incompatible or missing interface definitions:

* `depthai_ros_msgs`
* `mrpt_msgs`
* `mrpt_nav_interfaces`
* `rosbag2_test_msgdefs`
* `sick_scan_xd`

Therefore, the features have been commented out in the `Cargo.toml` file.
