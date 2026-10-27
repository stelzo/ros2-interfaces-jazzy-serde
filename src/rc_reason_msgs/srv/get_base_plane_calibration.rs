use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationRequest {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for GetBasePlaneCalibrationRequest {
    fn default() -> Self {
        GetBasePlaneCalibrationRequest {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBasePlaneCalibrationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBasePlaneCalibrationResponse {
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetBasePlaneCalibrationResponse {
    fn default() -> Self {
        GetBasePlaneCalibrationResponse {
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBasePlaneCalibrationResponse {}


pub struct GetBasePlaneCalibration;
