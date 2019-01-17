/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiPolicyV1beta1PodDisruptionBudget : PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiPolicyV1beta1PodDisruptionBudget {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  #[serde(rename = "spec")]
  spec: Option<::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec>,
  #[serde(rename = "status")]
  status: Option<::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetStatus>
}

impl IoK8sApiPolicyV1beta1PodDisruptionBudget {
  /// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
  pub fn new() -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    IoK8sApiPolicyV1beta1PodDisruptionBudget {
      api_version: None,
      kind: None,
      metadata: None,
      spec: None,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec) -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

  pub fn set_status(&mut self, status: ::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetStatus) -> IoK8sApiPolicyV1beta1PodDisruptionBudget {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::IoK8sApiPolicyV1beta1PodDisruptionBudgetStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



