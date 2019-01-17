/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
  /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
  #[serde(rename = "lastUpdateTime")]
  last_update_time: Option<String>,
  /// human readable message with details about the request state
  #[serde(rename = "message")]
  message: Option<String>,
  /// brief reason for the request state
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// request approval state, currently Approved or Denied.
  #[serde(rename = "type")]
  _type: String
}

impl IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
  pub fn new(_type: String) -> IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
    IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
      last_update_time: None,
      message: None,
      reason: None,
      _type: _type
    }
  }

  pub fn set_last_update_time(&mut self, last_update_time: String) {
    self.last_update_time = Some(last_update_time);
  }

  pub fn with_last_update_time(mut self, last_update_time: String) -> IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
    self.last_update_time = Some(last_update_time);
    self
  }

  pub fn last_update_time(&self) -> Option<&String> {
    self.last_update_time.as_ref()
  }

  pub fn reset_last_update_time(&mut self) {
    self.last_update_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> IoK8sApiCertificatesV1beta1CertificateSigningRequestCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



