/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus : SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
  /// EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
  #[serde(rename = "evaluationError")]
  evaluation_error: Option<String>,
  /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
  #[serde(rename = "incomplete")]
  incomplete: bool,
  /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
  #[serde(rename = "nonResourceRules")]
  non_resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1NonResourceRule>,
  /// ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
  #[serde(rename = "resourceRules")]
  resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1ResourceRule>
}

impl IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
  /// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.
  pub fn new(incomplete: bool, non_resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1NonResourceRule>, resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1ResourceRule>) -> IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
    IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
      evaluation_error: None,
      incomplete: incomplete,
      non_resource_rules: non_resource_rules,
      resource_rules: resource_rules
    }
  }

  pub fn set_evaluation_error(&mut self, evaluation_error: String) {
    self.evaluation_error = Some(evaluation_error);
  }

  pub fn with_evaluation_error(mut self, evaluation_error: String) -> IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
    self.evaluation_error = Some(evaluation_error);
    self
  }

  pub fn evaluation_error(&self) -> Option<&String> {
    self.evaluation_error.as_ref()
  }

  pub fn reset_evaluation_error(&mut self) {
    self.evaluation_error = None;
  }

  pub fn set_incomplete(&mut self, incomplete: bool) {
    self.incomplete = incomplete;
  }

  pub fn with_incomplete(mut self, incomplete: bool) -> IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
    self.incomplete = incomplete;
    self
  }

  pub fn incomplete(&self) -> &bool {
    &self.incomplete
  }


  pub fn set_non_resource_rules(&mut self, non_resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1NonResourceRule>) {
    self.non_resource_rules = non_resource_rules;
  }

  pub fn with_non_resource_rules(mut self, non_resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1NonResourceRule>) -> IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
    self.non_resource_rules = non_resource_rules;
    self
  }

  pub fn non_resource_rules(&self) -> &Vec<::models::IoK8sApiAuthorizationV1beta1NonResourceRule> {
    &self.non_resource_rules
  }


  pub fn set_resource_rules(&mut self, resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1ResourceRule>) {
    self.resource_rules = resource_rules;
  }

  pub fn with_resource_rules(mut self, resource_rules: Vec<::models::IoK8sApiAuthorizationV1beta1ResourceRule>) -> IoK8sApiAuthorizationV1beta1SubjectRulesReviewStatus {
    self.resource_rules = resource_rules;
    self
  }

  pub fn resource_rules(&self) -> &Vec<::models::IoK8sApiAuthorizationV1beta1ResourceRule> {
    &self.resource_rules
  }


}



