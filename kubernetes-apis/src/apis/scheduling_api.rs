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

pub struct SchedulingApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SchedulingApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SchedulingApiClient {
        SchedulingApiClient {
            configuration: configuration,
        }
    }
}

pub trait SchedulingApi {
    fn get_scheduling_api_group(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error>;
}


impl SchedulingApi for SchedulingApiClient {
    fn get_scheduling_api_group(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/apis/scheduling.k8s.io/?{}", configuration.base_path, query_string);

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
