/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1ReplicationControllerCondition : ReplicationControllerCondition describes the state of a replication controller at a certain point.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1ReplicationControllerCondition {
  /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<String>,
  /// A human readable message indicating details about the transition.
  #[serde(rename = "message")]
  message: Option<String>,
  /// The reason for the condition's last transition.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// Status of the condition, one of True, False, Unknown.
  #[serde(rename = "status")]
  status: String,
  /// Type of replication controller condition.
  #[serde(rename = "type")]
  _type: String
}

impl IoK8sApiCoreV1ReplicationControllerCondition {
  /// ReplicationControllerCondition describes the state of a replication controller at a certain point.
  pub fn new(status: String, _type: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    IoK8sApiCoreV1ReplicationControllerCondition {
      last_transition_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: String) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&String> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> IoK8sApiCoreV1ReplicationControllerCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



