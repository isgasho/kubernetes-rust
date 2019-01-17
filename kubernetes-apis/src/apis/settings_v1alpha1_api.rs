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

pub struct SettingsV1alpha1ApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SettingsV1alpha1ApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SettingsV1alpha1ApiClient {
        SettingsV1alpha1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait SettingsV1alpha1Api {
    fn create_settings_v1alpha1_namespaced_pod_preset(&self, namespace: &str, io_k8s_api_settings_v1alpha1_pod_preset: ::models::IoK8sApiSettingsV1alpha1PodPreset, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error>;
    fn delete_settings_v1alpha1_collection_namespaced_pod_preset(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1Status, Error>;
    fn delete_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions) -> Result<::models::IoK8sApimachineryPkgApisMetaV1Status, Error>;
    fn get_settings_v1alpha1_api_resources(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error>;
    fn list_settings_v1alpha1_namespaced_pod_preset(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPresetList, Error>;
    fn list_settings_v1alpha1_pod_preset_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPresetList, Error>;
    fn patch_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error>;
    fn read_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error>;
    fn replace_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, io_k8s_api_settings_v1alpha1_pod_preset: ::models::IoK8sApiSettingsV1alpha1PodPreset, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error>;
    fn watch_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error>;
    fn watch_settings_v1alpha1_namespaced_pod_preset_list(&self, namespace: &str, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error>;
    fn watch_settings_v1alpha1_pod_preset_list_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error>;
}


impl SettingsV1alpha1Api for SettingsV1alpha1ApiClient {
    fn create_settings_v1alpha1_namespaced_pod_preset(&self, namespace: &str, io_k8s_api_settings_v1alpha1_pod_preset: ::models::IoK8sApiSettingsV1alpha1PodPreset, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("dryRun", &dry_run.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets?{}", configuration.base_path, query_string, namespace=namespace);

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
        

        req_builder = req_builder.json(&io_k8s_api_settings_v1alpha1_pod_preset);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_settings_v1alpha1_collection_namespaced_pod_preset(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1Status, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets?{}", configuration.base_path, query_string, namespace=namespace);

        let mut req_builder = client.delete(uri_str.as_str());

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

    fn delete_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str, io_k8s_apimachinery_pkg_apis_meta_v1_delete_options: ::models::IoK8sApimachineryPkgApisMetaV1DeleteOptions) -> Result<::models::IoK8sApimachineryPkgApisMetaV1Status, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("dryRun", &dry_run.to_string());
            query.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
            query.append_pair("orphanDependents", &orphan_dependents.to_string());
            query.append_pair("propagationPolicy", &propagation_policy.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets/{name}?{}", configuration.base_path, query_string, name=name, namespace=namespace);

        let mut req_builder = client.delete(uri_str.as_str());

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
        

        req_builder = req_builder.json(&io_k8s_apimachinery_pkg_apis_meta_v1_delete_options);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_settings_v1alpha1_api_resources(&self, ) -> Result<::models::IoK8sApimachineryPkgApisMetaV1ApiResourceList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/?{}", configuration.base_path, query_string);

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

    fn list_settings_v1alpha1_namespaced_pod_preset(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPresetList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets?{}", configuration.base_path, query_string, namespace=namespace);

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

    fn list_settings_v1alpha1_pod_preset_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPresetList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/podpresets?{}", configuration.base_path, query_string);

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

    fn patch_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, io_k8s_apimachinery_pkg_apis_meta_v1_patch: ::models::IoK8sApimachineryPkgApisMetaV1Patch, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("dryRun", &dry_run.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets/{name}?{}", configuration.base_path, query_string, name=name, namespace=namespace);

        let mut req_builder = client.patch(uri_str.as_str());

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
        

        req_builder = req_builder.json(&io_k8s_apimachinery_pkg_apis_meta_v1_patch);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn read_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("exact", &exact.to_string());
            query.append_pair("export", &export.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets/{name}?{}", configuration.base_path, query_string, name=name, namespace=namespace);

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

    fn replace_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, io_k8s_api_settings_v1alpha1_pod_preset: ::models::IoK8sApiSettingsV1alpha1PodPreset, pretty: &str, dry_run: &str) -> Result<::models::IoK8sApiSettingsV1alpha1PodPreset, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("dryRun", &dry_run.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/namespaces/{namespace}/podpresets/{name}?{}", configuration.base_path, query_string, name=name, namespace=namespace);

        let mut req_builder = client.put(uri_str.as_str());

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
        

        req_builder = req_builder.json(&io_k8s_api_settings_v1alpha1_pod_preset);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn watch_settings_v1alpha1_namespaced_pod_preset(&self, name: &str, namespace: &str, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/watch/namespaces/{namespace}/podpresets/{name}?{}", configuration.base_path, query_string, name=name, namespace=namespace);

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

    fn watch_settings_v1alpha1_namespaced_pod_preset_list(&self, namespace: &str, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/watch/namespaces/{namespace}/podpresets?{}", configuration.base_path, query_string, namespace=namespace);

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

    fn watch_settings_v1alpha1_pod_preset_list_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Result<::models::IoK8sApimachineryPkgApisMetaV1WatchEvent, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("continue", &_continue.to_string());
            query.append_pair("fieldSelector", &field_selector.to_string());
            query.append_pair("includeUninitialized", &include_uninitialized.to_string());
            query.append_pair("labelSelector", &label_selector.to_string());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("pretty", &pretty.to_string());
            query.append_pair("resourceVersion", &resource_version.to_string());
            query.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            query.append_pair("watch", &watch.to_string());

            query.finish()
        };
        let uri_str = format!("{}/apis/settings.k8s.io/v1alpha1/watch/podpresets?{}", configuration.base_path, query_string);

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
