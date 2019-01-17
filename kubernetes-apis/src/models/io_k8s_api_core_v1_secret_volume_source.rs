/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1SecretVolumeSource : Adapts a Secret into a volume.  The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1SecretVolumeSource {
  /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
  #[serde(rename = "defaultMode")]
  default_mode: Option<i32>,
  /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
  #[serde(rename = "items")]
  items: Option<Vec<::models::IoK8sApiCoreV1KeyToPath>>,
  /// Specify whether the Secret or it's keys must be defined
  #[serde(rename = "optional")]
  optional: Option<bool>,
  /// Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
  #[serde(rename = "secretName")]
  secret_name: Option<String>
}

impl IoK8sApiCoreV1SecretVolumeSource {
  /// Adapts a Secret into a volume.  The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.
  pub fn new() -> IoK8sApiCoreV1SecretVolumeSource {
    IoK8sApiCoreV1SecretVolumeSource {
      default_mode: None,
      items: None,
      optional: None,
      secret_name: None
    }
  }

  pub fn set_default_mode(&mut self, default_mode: i32) {
    self.default_mode = Some(default_mode);
  }

  pub fn with_default_mode(mut self, default_mode: i32) -> IoK8sApiCoreV1SecretVolumeSource {
    self.default_mode = Some(default_mode);
    self
  }

  pub fn default_mode(&self) -> Option<&i32> {
    self.default_mode.as_ref()
  }

  pub fn reset_default_mode(&mut self) {
    self.default_mode = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::IoK8sApiCoreV1KeyToPath>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::IoK8sApiCoreV1KeyToPath>) -> IoK8sApiCoreV1SecretVolumeSource {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::IoK8sApiCoreV1KeyToPath>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_optional(&mut self, optional: bool) {
    self.optional = Some(optional);
  }

  pub fn with_optional(mut self, optional: bool) -> IoK8sApiCoreV1SecretVolumeSource {
    self.optional = Some(optional);
    self
  }

  pub fn optional(&self) -> Option<&bool> {
    self.optional.as_ref()
  }

  pub fn reset_optional(&mut self) {
    self.optional = None;
  }

  pub fn set_secret_name(&mut self, secret_name: String) {
    self.secret_name = Some(secret_name);
  }

  pub fn with_secret_name(mut self, secret_name: String) -> IoK8sApiCoreV1SecretVolumeSource {
    self.secret_name = Some(secret_name);
    self
  }

  pub fn secret_name(&self) -> Option<&String> {
    self.secret_name.as_ref()
  }

  pub fn reset_secret_name(&mut self) {
    self.secret_name = None;
  }

}



