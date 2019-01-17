/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1DownwardApiProjection : Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1DownwardApiProjection {
  /// Items is a list of DownwardAPIVolume file
  #[serde(rename = "items")]
  items: Option<Vec<::models::IoK8sApiCoreV1DownwardApiVolumeFile>>
}

impl IoK8sApiCoreV1DownwardApiProjection {
  /// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
  pub fn new() -> IoK8sApiCoreV1DownwardApiProjection {
    IoK8sApiCoreV1DownwardApiProjection {
      items: None
    }
  }

  pub fn set_items(&mut self, items: Vec<::models::IoK8sApiCoreV1DownwardApiVolumeFile>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::IoK8sApiCoreV1DownwardApiVolumeFile>) -> IoK8sApiCoreV1DownwardApiProjection {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::IoK8sApiCoreV1DownwardApiVolumeFile>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

}



