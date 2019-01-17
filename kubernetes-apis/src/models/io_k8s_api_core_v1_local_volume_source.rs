/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1LocalVolumeSource : Local represents directly-attached storage with node affinity (Beta feature)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1LocalVolumeSource {
  /// Filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default value is to auto-select a fileystem if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// The full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
  #[serde(rename = "path")]
  path: String
}

impl IoK8sApiCoreV1LocalVolumeSource {
  /// Local represents directly-attached storage with node affinity (Beta feature)
  pub fn new(path: String) -> IoK8sApiCoreV1LocalVolumeSource {
    IoK8sApiCoreV1LocalVolumeSource {
      fs_type: None,
      path: path
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> IoK8sApiCoreV1LocalVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> IoK8sApiCoreV1LocalVolumeSource {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}



