/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoordinationV1beta1LeaseSpec : LeaseSpec is a specification of a Lease.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoordinationV1beta1LeaseSpec {
  /// MicroTime is version of Time with microsecond level precision.
  #[serde(rename = "acquireTime")]
  acquire_time: Option<String>,
  /// holderIdentity contains the identity of the holder of a current lease.
  #[serde(rename = "holderIdentity")]
  holder_identity: Option<String>,
  /// leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measure against time of last observed RenewTime.
  #[serde(rename = "leaseDurationSeconds")]
  lease_duration_seconds: Option<i32>,
  /// leaseTransitions is the number of transitions of a lease between holders.
  #[serde(rename = "leaseTransitions")]
  lease_transitions: Option<i32>,
  /// MicroTime is version of Time with microsecond level precision.
  #[serde(rename = "renewTime")]
  renew_time: Option<String>
}

impl IoK8sApiCoordinationV1beta1LeaseSpec {
  /// LeaseSpec is a specification of a Lease.
  pub fn new() -> IoK8sApiCoordinationV1beta1LeaseSpec {
    IoK8sApiCoordinationV1beta1LeaseSpec {
      acquire_time: None,
      holder_identity: None,
      lease_duration_seconds: None,
      lease_transitions: None,
      renew_time: None
    }
  }

  pub fn set_acquire_time(&mut self, acquire_time: String) {
    self.acquire_time = Some(acquire_time);
  }

  pub fn with_acquire_time(mut self, acquire_time: String) -> IoK8sApiCoordinationV1beta1LeaseSpec {
    self.acquire_time = Some(acquire_time);
    self
  }

  pub fn acquire_time(&self) -> Option<&String> {
    self.acquire_time.as_ref()
  }

  pub fn reset_acquire_time(&mut self) {
    self.acquire_time = None;
  }

  pub fn set_holder_identity(&mut self, holder_identity: String) {
    self.holder_identity = Some(holder_identity);
  }

  pub fn with_holder_identity(mut self, holder_identity: String) -> IoK8sApiCoordinationV1beta1LeaseSpec {
    self.holder_identity = Some(holder_identity);
    self
  }

  pub fn holder_identity(&self) -> Option<&String> {
    self.holder_identity.as_ref()
  }

  pub fn reset_holder_identity(&mut self) {
    self.holder_identity = None;
  }

  pub fn set_lease_duration_seconds(&mut self, lease_duration_seconds: i32) {
    self.lease_duration_seconds = Some(lease_duration_seconds);
  }

  pub fn with_lease_duration_seconds(mut self, lease_duration_seconds: i32) -> IoK8sApiCoordinationV1beta1LeaseSpec {
    self.lease_duration_seconds = Some(lease_duration_seconds);
    self
  }

  pub fn lease_duration_seconds(&self) -> Option<&i32> {
    self.lease_duration_seconds.as_ref()
  }

  pub fn reset_lease_duration_seconds(&mut self) {
    self.lease_duration_seconds = None;
  }

  pub fn set_lease_transitions(&mut self, lease_transitions: i32) {
    self.lease_transitions = Some(lease_transitions);
  }

  pub fn with_lease_transitions(mut self, lease_transitions: i32) -> IoK8sApiCoordinationV1beta1LeaseSpec {
    self.lease_transitions = Some(lease_transitions);
    self
  }

  pub fn lease_transitions(&self) -> Option<&i32> {
    self.lease_transitions.as_ref()
  }

  pub fn reset_lease_transitions(&mut self) {
    self.lease_transitions = None;
  }

  pub fn set_renew_time(&mut self, renew_time: String) {
    self.renew_time = Some(renew_time);
  }

  pub fn with_renew_time(mut self, renew_time: String) -> IoK8sApiCoordinationV1beta1LeaseSpec {
    self.renew_time = Some(renew_time);
    self
  }

  pub fn renew_time(&self) -> Option<&String> {
    self.renew_time.as_ref()
  }

  pub fn reset_renew_time(&mut self) {
    self.renew_time = None;
  }

}



