use crate::api::ApiClient;
use crate::components::status_card::StatusCard;
use crate::components::transaction_list::TransactionList;
use crate::models::{NodeStatus, Transaction};
use crate::utils::format_amount;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let status = use_state(|| None::<NodeStatus>);
    let recent_transactions = use_state(|| Vec::<Transaction>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let _interval = use_state(|| {
        let status_clone = status.clone();
        let loading_clone = loading.clone();
        let error_clone = error.clone();
        
        Interval::new(10000, move || {
            let status_clone = status_clone.clone();
            let loading_clone = loading_clone.clone();
            let error_clone = error_clone.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get_status().await {
                    Ok(new_status) => {
                        status_clone.set(Some(new_status));
                        loading_clone.set(false);
                    }
                    Err(e) => {
                        error_clone.set(Some(e.to_string()));
                        loading_clone.set(false);
                    }
                }
            });
        })
    });
    
    // Initial data load
    {
        let status = status.clone();
        let recent_transactions = recent_transactions.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                loading.set(true);
                
                wasm_bindgen_futures::spawn_local(async move {
                    // Load status
                    match ApiClient::get_status().await {
                        Ok(new_status) => {
                            status.set(Some(new_status));
                            
                            // Load recent transactions
                            match ApiClient::get_blocks(None, Some(5)).await {
                                Ok(blocks) => {
                                    let mut txs = Vec::new();
                                    
                                    for block in blocks {
                                        for tx_id in block.transactions {
                                            if let Ok(tx) = ApiClient::get_transaction(&tx_id).await {
                                                txs.push(tx);
                                                if txs.len() >= 5 {
                                                    break;
                                                }
                                            }
                                        }
                                        
                                        if txs.len() >= 5 {
                                            break;
                                        }
                                    }
                                    
                                    recent_transactions.set(txs);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to load blocks: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
                        }
                    }
                    
                    loading.set(false);
                });
                
                || ()
            },
            (),
        );
    }
    
    html! {
        <div>
            <h1 class="mb-4">{"Dashboard"}</h1>
            
            if let Some(e) = (*error).clone() {
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {e}
                </div>
            }
            
            <div class="row mb-4">
                <div class="col-md-3">
                    <StatusCard 
                        title="Latest Block" 
                        value={
                            if let Some(s) = (*status).clone() {
                                s.latest_block_height.to_string()
                            } else {
                                "...".to_string()
                            }
                        }
                        icon="box"
                        color="primary"
                    />
                </div>
                <div class="col-md-3">
                    <StatusCard 
                        title="Connected Peers" 
                        value={
                            if let Some(s) = (*status).clone() {
                                s.connected_peers.to_string()
                            } else {
                                "...".to_string()
                            }
                        }
                        icon="people"
                        color="success"
                    />
                </div>
                <div class="col-md-3">
                    <StatusCard 
                        title="Pending Transactions" 
                        value={
                            if let Some(s) = (*status).clone() {
                                s.pending_transactions.to_string()
                            } else {
                                "...".to_string()
                            }
                        }
                        icon="hourglass-split"
                        color="warning"
                    />
                </div>
                <div class="col-md-3">
                    <StatusCard 
                        title="Uptime" 
                        value={
                            if let Some(s) = (*status).clone() {
                                format!("{} sec", s.uptime_seconds)
                            } else {
                                "...".to_string()
                            }
                        }
                        icon="clock-history"
                        color="info"
                    />
                </div>
            </div>
            
            <div class="row">
                <div class="col-md-12">
                    <div class="card">
                        <div class="card-header d-flex justify-content-between align-items-center">
                            <span>{"Recent Transactions"}</span>
                            <a href="/transactions" class="btn btn-sm btn-outline-primary">{"View All"}</a>
                        </div>
                        <div class="card-body">
                            <TransactionList 
                                transactions={(*recent_transactions).clone()} 
                                loading={*loading} 
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}