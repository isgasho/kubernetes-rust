/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiRbacV1Subject : Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiRbacV1Subject {
  /// APIGroup holds the API group of the referenced subject. Defaults to \"\" for ServiceAccount subjects. Defaults to \"rbac.authorization.k8s.io\" for User and Group subjects.
  #[serde(rename = "apiGroup")]
  api_group: Option<String>,
  /// Kind of object being referenced. Values defined by this API group are \"User\", \"Group\", and \"ServiceAccount\". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
  #[serde(rename = "kind")]
  kind: String,
  /// Name of the object being referenced.
  #[serde(rename = "name")]
  name: String,
  /// Namespace of the referenced object.  If the object kind is non-namespace, such as \"User\" or \"Group\", and this value is not empty the Authorizer should report an error.
  #[serde(rename = "namespace")]
  namespace: Option<String>
}

impl IoK8sApiRbacV1Subject {
  /// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
  pub fn new(kind: String, name: String) -> IoK8sApiRbacV1Subject {
    IoK8sApiRbacV1Subject {
      api_group: None,
      kind: kind,
      name: name,
      namespace: None
    }
  }

  pub fn set_api_group(&mut self, api_group: String) {
    self.api_group = Some(api_group);
  }

  pub fn with_api_group(mut self, api_group: String) -> IoK8sApiRbacV1Subject {
    self.api_group = Some(api_group);
    self
  }

  pub fn api_group(&self) -> Option<&String> {
    self.api_group.as_ref()
  }

  pub fn reset_api_group(&mut self) {
    self.api_group = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = kind;
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApiRbacV1Subject {
    self.kind = kind;
    self
  }

  pub fn kind(&self) -> &String {
    &self.kind
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApiRbacV1Subject {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = Some(namespace);
  }

  pub fn with_namespace(mut self, namespace: String) -> IoK8sApiRbacV1Subject {
    self.namespace = Some(namespace);
    self
  }

  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  pub fn reset_namespace(&mut self) {
    self.namespace = None;
  }

}



