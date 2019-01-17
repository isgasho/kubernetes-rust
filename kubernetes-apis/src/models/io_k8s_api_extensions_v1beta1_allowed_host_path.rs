/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1AllowedHostPath : AllowedHostPath defines the host volume conditions that will be enabled by a policy for pods to use. It requires the path prefix to be defined. Deprecated: use AllowedHostPath from policy API Group instead.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1AllowedHostPath {
  /// pathPrefix is the path prefix that the host volume must match. It does not support `*`. Trailing slashes are trimmed when validating the path prefix with a host path.  Examples: `/foo` would allow `/foo`, `/foo/` and `/foo/bar` `/foo` would not allow `/food` or `/etc/foo`
  #[serde(rename = "pathPrefix")]
  path_prefix: Option<String>,
  /// when set to true, will allow host volumes matching the pathPrefix only if all volume mounts are readOnly.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl IoK8sApiExtensionsV1beta1AllowedHostPath {
  /// AllowedHostPath defines the host volume conditions that will be enabled by a policy for pods to use. It requires the path prefix to be defined. Deprecated: use AllowedHostPath from policy API Group instead.
  pub fn new() -> IoK8sApiExtensionsV1beta1AllowedHostPath {
    IoK8sApiExtensionsV1beta1AllowedHostPath {
      path_prefix: None,
      read_only: None
    }
  }

  pub fn set_path_prefix(&mut self, path_prefix: String) {
    self.path_prefix = Some(path_prefix);
  }

  pub fn with_path_prefix(mut self, path_prefix: String) -> IoK8sApiExtensionsV1beta1AllowedHostPath {
    self.path_prefix = Some(path_prefix);
    self
  }

  pub fn path_prefix(&self) -> Option<&String> {
    self.path_prefix.as_ref()
  }

  pub fn reset_path_prefix(&mut self) {
    self.path_prefix = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> IoK8sApiExtensionsV1beta1AllowedHostPath {
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



