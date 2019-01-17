/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAutoscalingV2beta2ResourceMetricStatus : ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
  #[serde(rename = "current")]
  current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus,
  /// Name is the name of the resource in question.
  #[serde(rename = "name")]
  name: String
}

impl IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
  /// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.
  pub fn new(current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus, name: String) -> IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
    IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
      current: current,
      name: name
    }
  }

  pub fn set_current(&mut self, current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus) {
    self.current = current;
  }

  pub fn with_current(mut self, current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus) -> IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
    self.current = current;
    self
  }

  pub fn current(&self) -> &::models::IoK8sApiAutoscalingV2beta2MetricValueStatus {
    &self.current
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApiAutoscalingV2beta2ResourceMetricStatus {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



