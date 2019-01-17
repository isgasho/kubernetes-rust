/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview : SelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a spec.namespace means \"in all namespaces\".  Self is a special case, because users should always be able to check whether they can perform an action

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  #[serde(rename = "spec")]
  spec: ::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReviewSpec,
  #[serde(rename = "status")]
  status: Option<::models::IoK8sApiAuthorizationV1beta1SubjectAccessReviewStatus>
}

impl IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
  /// SelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a spec.namespace means \"in all namespaces\".  Self is a special case, because users should always be able to check whether they can perform an action
  pub fn new(spec: ::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReviewSpec) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
    IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
      api_version: None,
      kind: None,
      metadata: None,
      spec: spec,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
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

  pub fn with_kind(mut self, kind: String) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
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

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReviewSpec) {
    self.spec = spec;
  }

  pub fn with_spec(mut self, spec: ::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReviewSpec) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
    self.spec = spec;
    self
  }

  pub fn spec(&self) -> &::models::IoK8sApiAuthorizationV1beta1SelfSubjectAccessReviewSpec {
    &self.spec
  }


  pub fn set_status(&mut self, status: ::models::IoK8sApiAuthorizationV1beta1SubjectAccessReviewStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::IoK8sApiAuthorizationV1beta1SubjectAccessReviewStatus) -> IoK8sApiAuthorizationV1beta1SelfSubjectAccessReview {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::IoK8sApiAuthorizationV1beta1SubjectAccessReviewStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



