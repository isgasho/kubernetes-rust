/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiBatchV1beta1CronJobStatus : CronJobStatus represents the current state of a cron job.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiBatchV1beta1CronJobStatus {
  /// A list of pointers to currently running jobs.
  #[serde(rename = "active")]
  active: Option<Vec<::models::IoK8sApiCoreV1ObjectReference>>,
  /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
  #[serde(rename = "lastScheduleTime")]
  last_schedule_time: Option<String>
}

impl IoK8sApiBatchV1beta1CronJobStatus {
  /// CronJobStatus represents the current state of a cron job.
  pub fn new() -> IoK8sApiBatchV1beta1CronJobStatus {
    IoK8sApiBatchV1beta1CronJobStatus {
      active: None,
      last_schedule_time: None
    }
  }

  pub fn set_active(&mut self, active: Vec<::models::IoK8sApiCoreV1ObjectReference>) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: Vec<::models::IoK8sApiCoreV1ObjectReference>) -> IoK8sApiBatchV1beta1CronJobStatus {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&Vec<::models::IoK8sApiCoreV1ObjectReference>> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_last_schedule_time(&mut self, last_schedule_time: String) {
    self.last_schedule_time = Some(last_schedule_time);
  }

  pub fn with_last_schedule_time(mut self, last_schedule_time: String) -> IoK8sApiBatchV1beta1CronJobStatus {
    self.last_schedule_time = Some(last_schedule_time);
    self
  }

  pub fn last_schedule_time(&self) -> Option<&String> {
    self.last_schedule_time.as_ref()
  }

  pub fn reset_last_schedule_time(&mut self) {
    self.last_schedule_time = None;
  }

}



