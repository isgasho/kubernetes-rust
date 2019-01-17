/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiBatchV1beta1CronJobSpec : CronJobSpec describes how the job execution will look like and when it will actually run.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiBatchV1beta1CronJobSpec {
  /// Specifies how to treat concurrent executions of a Job. Valid values are: - \"Allow\" (default): allows CronJobs to run concurrently; - \"Forbid\": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - \"Replace\": cancels currently running job and replaces it with a new one
  #[serde(rename = "concurrencyPolicy")]
  concurrency_policy: Option<String>,
  /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
  #[serde(rename = "failedJobsHistoryLimit")]
  failed_jobs_history_limit: Option<i32>,
  #[serde(rename = "jobTemplate")]
  job_template: ::models::IoK8sApiBatchV1beta1JobTemplateSpec,
  /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
  #[serde(rename = "schedule")]
  schedule: String,
  /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
  #[serde(rename = "startingDeadlineSeconds")]
  starting_deadline_seconds: Option<i64>,
  /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. Defaults to 3.
  #[serde(rename = "successfulJobsHistoryLimit")]
  successful_jobs_history_limit: Option<i32>,
  /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
  #[serde(rename = "suspend")]
  suspend: Option<bool>
}

impl IoK8sApiBatchV1beta1CronJobSpec {
  /// CronJobSpec describes how the job execution will look like and when it will actually run.
  pub fn new(job_template: ::models::IoK8sApiBatchV1beta1JobTemplateSpec, schedule: String) -> IoK8sApiBatchV1beta1CronJobSpec {
    IoK8sApiBatchV1beta1CronJobSpec {
      concurrency_policy: None,
      failed_jobs_history_limit: None,
      job_template: job_template,
      schedule: schedule,
      starting_deadline_seconds: None,
      successful_jobs_history_limit: None,
      suspend: None
    }
  }

  pub fn set_concurrency_policy(&mut self, concurrency_policy: String) {
    self.concurrency_policy = Some(concurrency_policy);
  }

  pub fn with_concurrency_policy(mut self, concurrency_policy: String) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.concurrency_policy = Some(concurrency_policy);
    self
  }

  pub fn concurrency_policy(&self) -> Option<&String> {
    self.concurrency_policy.as_ref()
  }

  pub fn reset_concurrency_policy(&mut self) {
    self.concurrency_policy = None;
  }

  pub fn set_failed_jobs_history_limit(&mut self, failed_jobs_history_limit: i32) {
    self.failed_jobs_history_limit = Some(failed_jobs_history_limit);
  }

  pub fn with_failed_jobs_history_limit(mut self, failed_jobs_history_limit: i32) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.failed_jobs_history_limit = Some(failed_jobs_history_limit);
    self
  }

  pub fn failed_jobs_history_limit(&self) -> Option<&i32> {
    self.failed_jobs_history_limit.as_ref()
  }

  pub fn reset_failed_jobs_history_limit(&mut self) {
    self.failed_jobs_history_limit = None;
  }

  pub fn set_job_template(&mut self, job_template: ::models::IoK8sApiBatchV1beta1JobTemplateSpec) {
    self.job_template = job_template;
  }

  pub fn with_job_template(mut self, job_template: ::models::IoK8sApiBatchV1beta1JobTemplateSpec) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.job_template = job_template;
    self
  }

  pub fn job_template(&self) -> &::models::IoK8sApiBatchV1beta1JobTemplateSpec {
    &self.job_template
  }


  pub fn set_schedule(&mut self, schedule: String) {
    self.schedule = schedule;
  }

  pub fn with_schedule(mut self, schedule: String) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.schedule = schedule;
    self
  }

  pub fn schedule(&self) -> &String {
    &self.schedule
  }


  pub fn set_starting_deadline_seconds(&mut self, starting_deadline_seconds: i64) {
    self.starting_deadline_seconds = Some(starting_deadline_seconds);
  }

  pub fn with_starting_deadline_seconds(mut self, starting_deadline_seconds: i64) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.starting_deadline_seconds = Some(starting_deadline_seconds);
    self
  }

  pub fn starting_deadline_seconds(&self) -> Option<&i64> {
    self.starting_deadline_seconds.as_ref()
  }

  pub fn reset_starting_deadline_seconds(&mut self) {
    self.starting_deadline_seconds = None;
  }

  pub fn set_successful_jobs_history_limit(&mut self, successful_jobs_history_limit: i32) {
    self.successful_jobs_history_limit = Some(successful_jobs_history_limit);
  }

  pub fn with_successful_jobs_history_limit(mut self, successful_jobs_history_limit: i32) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.successful_jobs_history_limit = Some(successful_jobs_history_limit);
    self
  }

  pub fn successful_jobs_history_limit(&self) -> Option<&i32> {
    self.successful_jobs_history_limit.as_ref()
  }

  pub fn reset_successful_jobs_history_limit(&mut self) {
    self.successful_jobs_history_limit = None;
  }

  pub fn set_suspend(&mut self, suspend: bool) {
    self.suspend = Some(suspend);
  }

  pub fn with_suspend(mut self, suspend: bool) -> IoK8sApiBatchV1beta1CronJobSpec {
    self.suspend = Some(suspend);
    self
  }

  pub fn suspend(&self) -> Option<&bool> {
    self.suspend.as_ref()
  }

  pub fn reset_suspend(&mut self) {
    self.suspend = None;
  }

}



