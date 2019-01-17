/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1IngressTls : IngressTLS describes the transport layer security associated with an Ingress.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1IngressTls {
  /// Hosts are a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
  #[serde(rename = "hosts")]
  hosts: Option<Vec<String>>,
  /// SecretName is the name of the secret used to terminate SSL traffic on 443. Field is left optional to allow SSL routing based on SNI hostname alone. If the SNI host in a listener conflicts with the \"Host\" header field used by an IngressRule, the SNI host is used for termination and value of the Host header is used for routing.
  #[serde(rename = "secretName")]
  secret_name: Option<String>
}

impl IoK8sApiExtensionsV1beta1IngressTls {
  /// IngressTLS describes the transport layer security associated with an Ingress.
  pub fn new() -> IoK8sApiExtensionsV1beta1IngressTls {
    IoK8sApiExtensionsV1beta1IngressTls {
      hosts: None,
      secret_name: None
    }
  }

  pub fn set_hosts(&mut self, hosts: Vec<String>) {
    self.hosts = Some(hosts);
  }

  pub fn with_hosts(mut self, hosts: Vec<String>) -> IoK8sApiExtensionsV1beta1IngressTls {
    self.hosts = Some(hosts);
    self
  }

  pub fn hosts(&self) -> Option<&Vec<String>> {
    self.hosts.as_ref()
  }

  pub fn reset_hosts(&mut self) {
    self.hosts = None;
  }

  pub fn set_secret_name(&mut self, secret_name: String) {
    self.secret_name = Some(secret_name);
  }

  pub fn with_secret_name(mut self, secret_name: String) -> IoK8sApiExtensionsV1beta1IngressTls {
    self.secret_name = Some(secret_name);
    self
  }

  pub fn secret_name(&self) -> Option<&String> {
    self.secret_name.as_ref()
  }

  pub fn reset_secret_name(&mut self) {
    self.secret_name = None;
  }

}



