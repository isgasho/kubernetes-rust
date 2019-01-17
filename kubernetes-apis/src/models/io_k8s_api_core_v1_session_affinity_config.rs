/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1SessionAffinityConfig : SessionAffinityConfig represents the configurations of session affinity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1SessionAffinityConfig {
  #[serde(rename = "clientIP")]
  client_ip: Option<::models::IoK8sApiCoreV1ClientIpConfig>
}

impl IoK8sApiCoreV1SessionAffinityConfig {
  /// SessionAffinityConfig represents the configurations of session affinity.
  pub fn new() -> IoK8sApiCoreV1SessionAffinityConfig {
    IoK8sApiCoreV1SessionAffinityConfig {
      client_ip: None
    }
  }

  pub fn set_client_ip(&mut self, client_ip: ::models::IoK8sApiCoreV1ClientIpConfig) {
    self.client_ip = Some(client_ip);
  }

  pub fn with_client_ip(mut self, client_ip: ::models::IoK8sApiCoreV1ClientIpConfig) -> IoK8sApiCoreV1SessionAffinityConfig {
    self.client_ip = Some(client_ip);
    self
  }

  pub fn client_ip(&self) -> Option<&::models::IoK8sApiCoreV1ClientIpConfig> {
    self.client_ip.as_ref()
  }

  pub fn reset_client_ip(&mut self) {
    self.client_ip = None;
  }

}



