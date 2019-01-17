/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiStorageV1alpha1VolumeError : VolumeError captures an error encountered during a volume operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiStorageV1alpha1VolumeError {
  /// String detailing the error encountered during Attach or Detach operation. This string maybe logged, so it should not contain sensitive information.
  #[serde(rename = "message")]
  message: Option<String>,
  /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
  #[serde(rename = "time")]
  time: Option<String>
}

impl IoK8sApiStorageV1alpha1VolumeError {
  /// VolumeError captures an error encountered during a volume operation.
  pub fn new() -> IoK8sApiStorageV1alpha1VolumeError {
    IoK8sApiStorageV1alpha1VolumeError {
      message: None,
      time: None
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> IoK8sApiStorageV1alpha1VolumeError {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_time(&mut self, time: String) {
    self.time = Some(time);
  }

  pub fn with_time(mut self, time: String) -> IoK8sApiStorageV1alpha1VolumeError {
    self.time = Some(time);
    self
  }

  pub fn time(&self) -> Option<&String> {
    self.time.as_ref()
  }

  pub fn reset_time(&mut self) {
    self.time = None;
  }

}



