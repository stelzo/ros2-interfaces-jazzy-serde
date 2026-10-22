use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwistWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl Default for TwistWithCovarianceStamped {
    fn default() -> Self {
        TwistWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TwistWithCovarianceStamped {}
