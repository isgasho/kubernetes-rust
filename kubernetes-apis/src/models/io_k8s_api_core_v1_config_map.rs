/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1ConfigMap : ConfigMap holds configuration data for pods to consume.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1ConfigMap {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// BinaryData contains the binary data. Each key must consist of alphanumeric characters, '-', '_' or '.'. BinaryData can contain byte sequences that are not in the UTF-8 range. The keys stored in BinaryData must not overlap with the ones in the Data field, this is enforced during validation process. Using this field will require 1.10+ apiserver and kubelet.
  #[serde(rename = "binaryData")]
  binary_data: Option<::std::collections::HashMap<String, String>>,
  /// Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'. Values with non-UTF-8 byte sequences must use the BinaryData field. The keys stored in Data must not overlap with the keys in the BinaryData field, this is enforced during validation process.
  #[serde(rename = "data")]
  data: Option<::std::collections::HashMap<String, String>>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>
}

impl IoK8sApiCoreV1ConfigMap {
  /// ConfigMap holds configuration data for pods to consume.
  pub fn new() -> IoK8sApiCoreV1ConfigMap {
    IoK8sApiCoreV1ConfigMap {
      api_version: None,
      binary_data: None,
      data: None,
      kind: None,
      metadata: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApiCoreV1ConfigMap {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_binary_data(&mut self, binary_data: ::std::collections::HashMap<String, String>) {
    self.binary_data = Some(binary_data);
  }

  pub fn with_binary_data(mut self, binary_data: ::std::collections::HashMap<String, String>) -> IoK8sApiCoreV1ConfigMap {
    self.binary_data = Some(binary_data);
    self
  }

  pub fn binary_data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.binary_data.as_ref()
  }

  pub fn reset_binary_data(&mut self) {
    self.binary_data = None;
  }

  pub fn set_data(&mut self, data: ::std::collections::HashMap<String, String>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::std::collections::HashMap<String, String>) -> IoK8sApiCoreV1ConfigMap {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApiCoreV1ConfigMap {
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

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sApiCoreV1ConfigMap {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}



