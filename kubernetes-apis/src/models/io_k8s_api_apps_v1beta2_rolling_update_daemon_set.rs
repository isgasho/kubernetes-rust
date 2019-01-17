/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAppsV1beta2RollingUpdateDaemonSet : Spec to control the desired behavior of daemon set rolling update.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAppsV1beta2RollingUpdateDaemonSet {
  /// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
  #[serde(rename = "maxUnavailable")]
  max_unavailable: Option<String>
}

impl IoK8sApiAppsV1beta2RollingUpdateDaemonSet {
  /// Spec to control the desired behavior of daemon set rolling update.
  pub fn new() -> IoK8sApiAppsV1beta2RollingUpdateDaemonSet {
    IoK8sApiAppsV1beta2RollingUpdateDaemonSet {
      max_unavailable: None
    }
  }

  pub fn set_max_unavailable(&mut self, max_unavailable: String) {
    self.max_unavailable = Some(max_unavailable);
  }

  pub fn with_max_unavailable(mut self, max_unavailable: String) -> IoK8sApiAppsV1beta2RollingUpdateDaemonSet {
    self.max_unavailable = Some(max_unavailable);
    self
  }

  pub fn max_unavailable(&self) -> Option<&String> {
    self.max_unavailable.as_ref()
  }

  pub fn reset_max_unavailable(&mut self) {
    self.max_unavailable = None;
  }

}



