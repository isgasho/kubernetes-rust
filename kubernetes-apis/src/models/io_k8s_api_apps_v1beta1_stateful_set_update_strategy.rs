/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAppsV1beta1StatefulSetUpdateStrategy : StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
  #[serde(rename = "rollingUpdate")]
  rolling_update: Option<::models::IoK8sApiAppsV1beta1RollingUpdateStatefulSetStrategy>,
  /// Type indicates the type of the StatefulSetUpdateStrategy.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
  /// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
  pub fn new() -> IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
    IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
      rolling_update: None,
      _type: None
    }
  }

  pub fn set_rolling_update(&mut self, rolling_update: ::models::IoK8sApiAppsV1beta1RollingUpdateStatefulSetStrategy) {
    self.rolling_update = Some(rolling_update);
  }

  pub fn with_rolling_update(mut self, rolling_update: ::models::IoK8sApiAppsV1beta1RollingUpdateStatefulSetStrategy) -> IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
    self.rolling_update = Some(rolling_update);
    self
  }

  pub fn rolling_update(&self) -> Option<&::models::IoK8sApiAppsV1beta1RollingUpdateStatefulSetStrategy> {
    self.rolling_update.as_ref()
  }

  pub fn reset_rolling_update(&mut self) {
    self.rolling_update = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> IoK8sApiAppsV1beta1StatefulSetUpdateStrategy {
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



