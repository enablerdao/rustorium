use crate::api::ApiClient;
use crate::components::transaction_list::TransactionList;
use crate::models::Transaction;
use yew::prelude::*;

#[function_component(TransactionsPage)]
pub fn transactions_page() -> Html {
    let transactions = use_state(|| Vec::<Transaction>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Load transactions
    {
        let transactions = transactions.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                loading.set(true);
                
                wasm_bindgen_futures::spawn_local(async move {
                    // Load blocks to get transaction IDs
                    match ApiClient::get_blocks(None, Some(10)).await {
                        Ok(blocks) => {
                            let mut txs = Vec::new();
                            
                            for block in blocks {
                                for tx_id in block.transactions {
                                    if let Ok(tx) = ApiClient::get_transaction(&tx_id).await {
                                        txs.push(tx);
                                    }
                                }
                            }
                            
                            transactions.set(txs);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
                            loading.set(false);
                        }
                    }
                });
                
                || ()
            },
            (),
        );
    }
    
    html! {
        <div>
            <h1 class="mb-4">{"Transactions"}</h1>
            
            if let Some(e) = (*error).clone() {
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {e}
                </div>
            }
            
            <div class="card">
                <div class="card-body">
                    <TransactionList transactions={(*transactions).clone()} loading={*loading} />
                </div>
            </div>
        </div>
    }
}