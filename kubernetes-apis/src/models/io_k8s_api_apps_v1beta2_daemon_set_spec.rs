/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAppsV1beta2DaemonSetSpec : DaemonSetSpec is the specification of a daemon set.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAppsV1beta2DaemonSetSpec {
  /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
  #[serde(rename = "minReadySeconds")]
  min_ready_seconds: Option<i32>,
  /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
  #[serde(rename = "revisionHistoryLimit")]
  revision_history_limit: Option<i32>,
  #[serde(rename = "selector")]
  selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector,
  #[serde(rename = "template")]
  template: ::models::IoK8sApiCoreV1PodTemplateSpec,
  #[serde(rename = "updateStrategy")]
  update_strategy: Option<::models::IoK8sApiAppsV1beta2DaemonSetUpdateStrategy>
}

impl IoK8sApiAppsV1beta2DaemonSetSpec {
  /// DaemonSetSpec is the specification of a daemon set.
  pub fn new(selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector, template: ::models::IoK8sApiCoreV1PodTemplateSpec) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    IoK8sApiAppsV1beta2DaemonSetSpec {
      min_ready_seconds: None,
      revision_history_limit: None,
      selector: selector,
      template: template,
      update_strategy: None
    }
  }

  pub fn set_min_ready_seconds(&mut self, min_ready_seconds: i32) {
    self.min_ready_seconds = Some(min_ready_seconds);
  }

  pub fn with_min_ready_seconds(mut self, min_ready_seconds: i32) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    self.min_ready_seconds = Some(min_ready_seconds);
    self
  }

  pub fn min_ready_seconds(&self) -> Option<&i32> {
    self.min_ready_seconds.as_ref()
  }

  pub fn reset_min_ready_seconds(&mut self) {
    self.min_ready_seconds = None;
  }

  pub fn set_revision_history_limit(&mut self, revision_history_limit: i32) {
    self.revision_history_limit = Some(revision_history_limit);
  }

  pub fn with_revision_history_limit(mut self, revision_history_limit: i32) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    self.revision_history_limit = Some(revision_history_limit);
    self
  }

  pub fn revision_history_limit(&self) -> Option<&i32> {
    self.revision_history_limit.as_ref()
  }

  pub fn reset_revision_history_limit(&mut self) {
    self.revision_history_limit = None;
  }

  pub fn set_selector(&mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.selector = selector;
  }

  pub fn with_selector(mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    self.selector = selector;
    self
  }

  pub fn selector(&self) -> &::models::IoK8sApimachineryPkgApisMetaV1LabelSelector {
    &self.selector
  }


  pub fn set_template(&mut self, template: ::models::IoK8sApiCoreV1PodTemplateSpec) {
    self.template = template;
  }

  pub fn with_template(mut self, template: ::models::IoK8sApiCoreV1PodTemplateSpec) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    self.template = template;
    self
  }

  pub fn template(&self) -> &::models::IoK8sApiCoreV1PodTemplateSpec {
    &self.template
  }


  pub fn set_update_strategy(&mut self, update_strategy: ::models::IoK8sApiAppsV1beta2DaemonSetUpdateStrategy) {
    self.update_strategy = Some(update_strategy);
  }

  pub fn with_update_strategy(mut self, update_strategy: ::models::IoK8sApiAppsV1beta2DaemonSetUpdateStrategy) -> IoK8sApiAppsV1beta2DaemonSetSpec {
    self.update_strategy = Some(update_strategy);
    self
  }

  pub fn update_strategy(&self) -> Option<&::models::IoK8sApiAppsV1beta2DaemonSetUpdateStrategy> {
    self.update_strategy.as_ref()
  }

  pub fn reset_update_strategy(&mut self) {
    self.update_strategy = None;
  }

}



