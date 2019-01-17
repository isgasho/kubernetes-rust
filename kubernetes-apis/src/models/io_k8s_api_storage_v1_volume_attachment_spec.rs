/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiStorageV1VolumeAttachmentSpec : VolumeAttachmentSpec is the specification of a VolumeAttachment request.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiStorageV1VolumeAttachmentSpec {
  /// Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
  #[serde(rename = "attacher")]
  attacher: String,
  /// The node that the volume should be attached to.
  #[serde(rename = "nodeName")]
  node_name: String,
  #[serde(rename = "source")]
  source: ::models::IoK8sApiStorageV1VolumeAttachmentSource
}

impl IoK8sApiStorageV1VolumeAttachmentSpec {
  /// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
  pub fn new(attacher: String, node_name: String, source: ::models::IoK8sApiStorageV1VolumeAttachmentSource) -> IoK8sApiStorageV1VolumeAttachmentSpec {
    IoK8sApiStorageV1VolumeAttachmentSpec {
      attacher: attacher,
      node_name: node_name,
      source: source
    }
  }

  pub fn set_attacher(&mut self, attacher: String) {
    self.attacher = attacher;
  }

  pub fn with_attacher(mut self, attacher: String) -> IoK8sApiStorageV1VolumeAttachmentSpec {
    self.attacher = attacher;
    self
  }

  pub fn attacher(&self) -> &String {
    &self.attacher
  }


  pub fn set_node_name(&mut self, node_name: String) {
    self.node_name = node_name;
  }

  pub fn with_node_name(mut self, node_name: String) -> IoK8sApiStorageV1VolumeAttachmentSpec {
    self.node_name = node_name;
    self
  }

  pub fn node_name(&self) -> &String {
    &self.node_name
  }


  pub fn set_source(&mut self, source: ::models::IoK8sApiStorageV1VolumeAttachmentSource) {
    self.source = source;
  }

  pub fn with_source(mut self, source: ::models::IoK8sApiStorageV1VolumeAttachmentSource) -> IoK8sApiStorageV1VolumeAttachmentSpec {
    self.source = source;
    self
  }

  pub fn source(&self) -> &::models::IoK8sApiStorageV1VolumeAttachmentSource {
    &self.source
  }


}



