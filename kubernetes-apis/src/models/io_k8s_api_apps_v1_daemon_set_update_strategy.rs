/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAppsV1DaemonSetUpdateStrategy : DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAppsV1DaemonSetUpdateStrategy {
  #[serde(rename = "rollingUpdate")]
  rolling_update: Option<::models::IoK8sApiAppsV1RollingUpdateDaemonSet>,
  /// Type of daemon set update. Can be \"RollingUpdate\" or \"OnDelete\". Default is RollingUpdate.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl IoK8sApiAppsV1DaemonSetUpdateStrategy {
  /// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
  pub fn new() -> IoK8sApiAppsV1DaemonSetUpdateStrategy {
    IoK8sApiAppsV1DaemonSetUpdateStrategy {
      rolling_update: None,
      _type: None
    }
  }

  pub fn set_rolling_update(&mut self, rolling_update: ::models::IoK8sApiAppsV1RollingUpdateDaemonSet) {
    self.rolling_update = Some(rolling_update);
  }

  pub fn with_rolling_update(mut self, rolling_update: ::models::IoK8sApiAppsV1RollingUpdateDaemonSet) -> IoK8sApiAppsV1DaemonSetUpdateStrategy {
    self.rolling_update = Some(rolling_update);
    self
  }

  pub fn rolling_update(&self) -> Option<&::models::IoK8sApiAppsV1RollingUpdateDaemonSet> {
    self.rolling_update.as_ref()
  }

  pub fn reset_rolling_update(&mut self) {
    self.rolling_update = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> IoK8sApiAppsV1DaemonSetUpdateStrategy {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



