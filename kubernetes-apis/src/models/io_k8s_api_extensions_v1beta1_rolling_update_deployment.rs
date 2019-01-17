/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1RollingUpdateDeployment : Spec to control the desired behavior of rolling update.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
  /// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
  #[serde(rename = "maxSurge")]
  max_surge: Option<String>,
  /// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
  #[serde(rename = "maxUnavailable")]
  max_unavailable: Option<String>
}

impl IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
  /// Spec to control the desired behavior of rolling update.
  pub fn new() -> IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
    IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
      max_surge: None,
      max_unavailable: None
    }
  }

  pub fn set_max_surge(&mut self, max_surge: String) {
    self.max_surge = Some(max_surge);
  }

  pub fn with_max_surge(mut self, max_surge: String) -> IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
    self.max_surge = Some(max_surge);
    self
  }

  pub fn max_surge(&self) -> Option<&String> {
    self.max_surge.as_ref()
  }

  pub fn reset_max_surge(&mut self) {
    self.max_surge = None;
  }

  pub fn set_max_unavailable(&mut self, max_unavailable: String) {
    self.max_unavailable = Some(max_unavailable);
  }

  pub fn with_max_unavailable(mut self, max_unavailable: String) -> IoK8sApiExtensionsV1beta1RollingUpdateDeployment {
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



