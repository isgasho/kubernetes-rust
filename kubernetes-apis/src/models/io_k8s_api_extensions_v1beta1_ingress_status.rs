/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1IngressStatus : IngressStatus describe the current state of the Ingress.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1IngressStatus {
  #[serde(rename = "loadBalancer")]
  load_balancer: Option<::models::IoK8sApiCoreV1LoadBalancerStatus>
}

impl IoK8sApiExtensionsV1beta1IngressStatus {
  /// IngressStatus describe the current state of the Ingress.
  pub fn new() -> IoK8sApiExtensionsV1beta1IngressStatus {
    IoK8sApiExtensionsV1beta1IngressStatus {
      load_balancer: None
    }
  }

  pub fn set_load_balancer(&mut self, load_balancer: ::models::IoK8sApiCoreV1LoadBalancerStatus) {
    self.load_balancer = Some(load_balancer);
  }

  pub fn with_load_balancer(mut self, load_balancer: ::models::IoK8sApiCoreV1LoadBalancerStatus) -> IoK8sApiExtensionsV1beta1IngressStatus {
    self.load_balancer = Some(load_balancer);
    self
  }

  pub fn load_balancer(&self) -> Option<&::models::IoK8sApiCoreV1LoadBalancerStatus> {
    self.load_balancer.as_ref()
  }

  pub fn reset_load_balancer(&mut self) {
    self.load_balancer = None;
  }

}



