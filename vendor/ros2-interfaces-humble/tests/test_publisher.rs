use ros2_client::{Context, MessageTypeName, Name, NodeName, NodeOptions};
use ros2_interfaces_humble::std_msgs;

#[test]
#[cfg(feature = "std_msgs")]
fn test_publisher() {
    let context = Context::new().unwrap();
    let mut node = context
        .new_node(
            NodeName::new("/rustdds", "rustdds_listener").unwrap(),
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();

    let topic = node
        .create_topic(
            &Name::new("/","topic").unwrap(),
            MessageTypeName::new("std_msgs", "String"),
            &ros2_client::DEFAULT_PUBLISHER_QOS,
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
