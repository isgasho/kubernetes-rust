/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1NetworkPolicySpec : DEPRECATED 1.9 - This group version of NetworkPolicySpec is deprecated by networking/v1/NetworkPolicySpec.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1NetworkPolicySpec {
  /// List of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
  #[serde(rename = "egress")]
  egress: Option<Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyEgressRule>>,
  /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default).
  #[serde(rename = "ingress")]
  ingress: Option<Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyIngressRule>>,
  #[serde(rename = "podSelector")]
  pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector,
  /// List of rule types that the NetworkPolicy relates to. Valid options are Ingress, Egress, or Ingress,Egress. If this field is not specified, it will default based on the existence of Ingress or Egress rules; policies that contain an Egress section are assumed to affect Egress, and all policies (whether or not they contain an Ingress section) are assumed to affect Ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes [ \"Egress\" ]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include \"Egress\" (since such a policy would not include an Egress section and would otherwise default to just [ \"Ingress\" ]). This field is beta-level in 1.8
  #[serde(rename = "policyTypes")]
  policy_types: Option<Vec<String>>
}

impl IoK8sApiExtensionsV1beta1NetworkPolicySpec {
  /// DEPRECATED 1.9 - This group version of NetworkPolicySpec is deprecated by networking/v1/NetworkPolicySpec.
  pub fn new(pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiExtensionsV1beta1NetworkPolicySpec {
    IoK8sApiExtensionsV1beta1NetworkPolicySpec {
      egress: None,
      ingress: None,
      pod_selector: pod_selector,
      policy_types: None
    }
  }

  pub fn set_egress(&mut self, egress: Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyEgressRule>) {
    self.egress = Some(egress);
  }

  pub fn with_egress(mut self, egress: Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyEgressRule>) -> IoK8sApiExtensionsV1beta1NetworkPolicySpec {
    self.egress = Some(egress);
    self
  }

  pub fn egress(&self) -> Option<&Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyEgressRule>> {
    self.egress.as_ref()
  }

  pub fn reset_egress(&mut self) {
    self.egress = None;
  }

  pub fn set_ingress(&mut self, ingress: Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyIngressRule>) {
    self.ingress = Some(ingress);
  }

  pub fn with_ingress(mut self, ingress: Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyIngressRule>) -> IoK8sApiExtensionsV1beta1NetworkPolicySpec {
    self.ingress = Some(ingress);
    self
  }

  pub fn ingress(&self) -> Option<&Vec<::models::IoK8sApiExtensionsV1beta1NetworkPolicyIngressRule>> {
    self.ingress.as_ref()
  }

  pub fn reset_ingress(&mut self) {
    self.ingress = None;
  }

  pub fn set_pod_selector(&mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.pod_selector = pod_selector;
  }

  pub fn with_pod_selector(mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiExtensionsV1beta1NetworkPolicySpec {
    self.pod_selector = pod_selector;
    self
  }

  pub fn pod_selector(&self) -> &::models::IoK8sApimachineryPkgApisMetaV1LabelSelector {
    &self.pod_selector
  }


  pub fn set_policy_types(&mut self, policy_types: Vec<String>) {
    self.policy_types = Some(policy_types);
  }

  pub fn with_policy_types(mut self, policy_types: Vec<String>) -> IoK8sApiExtensionsV1beta1NetworkPolicySpec {
    self.policy_types = Some(policy_types);
    self
  }

  pub fn policy_types(&self) -> Option<&Vec<String>> {
    self.policy_types.as_ref()
  }

  pub fn reset_policy_types(&mut self) {
    self.policy_types = None;
  }

}



