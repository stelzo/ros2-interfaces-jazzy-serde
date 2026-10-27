use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioRequest {
    pub scenario: crate::scenario_execution_interfaces::msg::Scenario,
}

impl Default for ExecuteScenarioRequest {
    fn default() -> Self {
        ExecuteScenarioRequest {
            scenario: crate::scenario_execution_interfaces::msg::Scenario::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ExecuteScenarioRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioResponse {
    pub result: bool,
}

impl Default for ExecuteScenarioResponse {
    fn default() -> Self {
        ExecuteScenarioResponse {
            result: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ExecuteScenarioResponse {}


pub struct ExecuteScenario;
