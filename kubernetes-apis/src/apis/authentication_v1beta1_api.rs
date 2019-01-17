/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct AuthenticationV1beta1ApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl AuthenticationV1beta1ApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> AuthenticationV1beta1ApiClient {
        AuthenticationV1beta1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthenticationV1beta1Api {
    fn create_authentication_v1beta1_token_review(&self, io_k8s_api_authentication_v1beta1_token_review: ::models::IoK8sApiAuthenticationV1beta1TokenReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Result<::models::IoK8sApiAuthenticationV1beta1TokenReview, Error>;
    fn get_authentication_v1beta1_api_resources(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error>;
}


impl AuthenticationV1beta1Api for AuthenticationV1beta1ApiClient {
    fn create_authentication_v1beta1_token_review(&self, io_k8s_api_authentication_v1beta1_token_review: ::models::IoK8sApiAuthenticationV1beta1TokenReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Result<::models::IoK8sApiAuthenticationV1beta1TokenReview, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("dryRun", &dry_run.to_string());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("pretty", &pretty.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/authentication.k8s.io/v1beta1/tokenreviews?{}", configuration.base_path, query_string);

        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("authorization", val);
        };
        

        req_builder = req_builder.json(&io_k8s_api_authentication_v1beta1_token_review);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_authentication_v1beta1_api_resources(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/apis/authentication.k8s.io/v1beta1/?{}", configuration.base_path, query_string);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("authorization", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
