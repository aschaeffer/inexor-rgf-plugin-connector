use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;
use waiter_di::*;

use crate::behaviour::relation::connector::Connector;
use crate::behaviour::relation::connector::CONNECTORS;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[wrapper]
pub struct ConnectorStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<Connector>>>,
);

#[waiter_di::provides]
fn create_connector_storage() -> ConnectorStorage {
    ConnectorStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ConnectorRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_connector(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_connector(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

// #[derive(Clone)]
pub struct ConnectorRelationBehaviourProviderImpl {
    connectors: ConnectorStorage,
}

interfaces!(ConnectorRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

#[component]
impl ConnectorRelationBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            connectors: create_connector_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ConnectorRelationBehaviourProvider for ConnectorRelationBehaviourProviderImpl {
    fn create_connector(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let mut type_name = relation_instance.type_name.clone();
        // TODO: remove
        debug!("relation type name {}", relation_instance.type_name.clone());
        let mut function = CONNECTORS.get(type_name.as_str());
        if function.is_none() {
            let connector_type_name = CONNECTORS
                .keys()
                .into_iter()
                .find(|connector_type_name| type_name.starts_with(*connector_type_name));
            if connector_type_name.is_some() {
                // TODO: remove
                debug!(
                    "Detected relation type name starts with {}",
                    connector_type_name.unwrap()
                );
                function = CONNECTORS.get(connector_type_name.unwrap());
                type_name = String::from(*connector_type_name.unwrap());
            }
        }
        let connector = match function {
            Some(function) => Some(Arc::new(Connector::from_relation(
                relation_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if connector.is_some() {
            self.connectors
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), connector.unwrap());
            debug!(
                "Added behaviour {} to relation instance {:?}",
                type_name, edge_key
            );
        }
    }

    fn remove_connector(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.connectors.0.write().unwrap().remove(&edge_key);
        debug!(
            "Removed behaviour connector to relation instance {:?}",
            edge_key
        );
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self.connectors.0.write().unwrap().contains_key(&edge_key) {
            self.connectors.0.write().unwrap().remove(&edge_key);
            debug!(
                "Removed behaviour connector to relation instance {:?}",
                edge_key
            );
        }
    }
}

impl RelationBehaviourProvider for ConnectorRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.create_connector(relation_instance.clone());
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.remove_connector(relation_instance.clone());
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}