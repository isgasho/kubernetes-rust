/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1GlusterfsPersistentVolumeSource : Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
  /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "endpoints")]
  endpoints: String,
  /// EndpointsNamespace is the namespace that contains Glusterfs endpoint. If this field is empty, the EndpointNamespace defaults to the same namespace as the bound PVC. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "endpointsNamespace")]
  endpoints_namespace: Option<String>,
  /// Path is the Glusterfs volume path. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "path")]
  path: String,
  /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
  /// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.
  pub fn new(endpoints: String, path: String) -> IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
    IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
      endpoints: endpoints,
      endpoints_namespace: None,
      path: path,
      read_only: None
    }
  }

  pub fn set_endpoints(&mut self, endpoints: String) {
    self.endpoints = endpoints;
  }

  pub fn with_endpoints(mut self, endpoints: String) -> IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
    self.endpoints = endpoints;
    self
  }

  pub fn endpoints(&self) -> &String {
    &self.endpoints
  }


  pub fn set_endpoints_namespace(&mut self, endpoints_namespace: String) {
    self.endpoints_namespace = Some(endpoints_namespace);
  }

  pub fn with_endpoints_namespace(mut self, endpoints_namespace: String) -> IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
    self.endpoints_namespace = Some(endpoints_namespace);
    self
  }

  pub fn endpoints_namespace(&self) -> Option<&String> {
    self.endpoints_namespace.as_ref()
  }

  pub fn reset_endpoints_namespace(&mut self) {
    self.endpoints_namespace = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sApiCoreV1GlusterfsPersistentVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

}



