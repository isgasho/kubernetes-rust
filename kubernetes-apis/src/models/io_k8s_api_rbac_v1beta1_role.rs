/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiRbacV1beta1Role : Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiRbacV1beta1Role {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  /// Rules holds all the PolicyRules for this Role
  #[serde(rename = "rules")]
  rules: Vec<::models::IoK8sApiRbacV1beta1PolicyRule>
}

impl IoK8sApiRbacV1beta1Role {
  /// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
  pub fn new(rules: Vec<::models::IoK8sApiRbacV1beta1PolicyRule>) -> IoK8sApiRbacV1beta1Role {
    IoK8sApiRbacV1beta1Role {
      api_version: None,
      kind: None,
      metadata: None,
      rules: rules
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApiRbacV1beta1Role {
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

  pub fn with_kind(mut self, kind: String) -> IoK8sApiRbacV1beta1Role {
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

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sApiRbacV1beta1Role {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_rules(&mut self, rules: Vec<::models::IoK8sApiRbacV1beta1PolicyRule>) {
    self.rules = rules;
  }

  pub fn with_rules(mut self, rules: Vec<::models::IoK8sApiRbacV1beta1PolicyRule>) -> IoK8sApiRbacV1beta1Role {
    self.rules = rules;
    self
  }

  pub fn rules(&self) -> &Vec<::models::IoK8sApiRbacV1beta1PolicyRule> {
    &self.rules
  }


}



