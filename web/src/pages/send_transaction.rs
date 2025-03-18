use crate::api::ApiClient;
use crate::models::{CreateTransactionRequest, Transaction};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(SendTransactionPage)]
pub fn send_transaction_page() -> Html {
    let sender = use_state(|| String::new());
    let recipient = use_state(|| String::new());
    let amount = use_state(|| String::new());
    let fee = use_state(|| String::new());
    let data = use_state(|| String::new());
    
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<Transaction>);
    
    let on_sender_change = {
        let sender = sender.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            sender.set(input.value());
        })
    };
    
    let on_recipient_change = {
        let recipient = recipient.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            recipient.set(input.value());
        })
    };
    
    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            amount.set(input.value());
        })
    };
    
    let on_fee_change = {
        let fee = fee.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            fee.set(input.value());
        })
    };
    
    let on_data_change = {
        let data = data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            data.set(input.value());
        })
    };
    
    let on_submit = {
        let sender = sender.clone();
        let recipient = recipient.clone();
        let amount = amount.clone();
        let fee = fee.clone();
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            
            // Validate inputs
            if sender.is_empty() {
                error.set(Some("Sender address is required".to_string()));
                return;
            }
            
            if recipient.is_empty() {
                error.set(Some("Recipient address is required".to_string()));
                return;
            }
            
            let amount_val = match amount.parse::<u64>() {
                Ok(val) => val,
                Err(_) => {
                    error.set(Some("Amount must be a valid number".to_string()));
                    return;
                }
            };
            
            let fee_val = if fee.is_empty() {
                1 // Default fee
            } else {
                match fee.parse::<u64>() {
                    Ok(val) => val,
                    Err(_) => {
                        error.set(Some("Fee must be a valid number".to_string()));
                        return;
                    }
                }
            };
            
            // Create request
            let request = CreateTransactionRequest {
                sender: (*sender).clone(),
                recipient: (*recipient).clone(),
                amount: amount_val,
                fee: fee_val,
                nonce: None,
                data: if data.is_empty() { None } else { Some((*data).clone()) },
            };
            
            loading.set(true);
            error.set(None);
            success.set(None);
            
            let loading_clone = loading.clone();
            let error_clone = error.clone();
            let success_clone = success.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::create_transaction(request).await {
                    Ok(tx) => {
                        success_clone.set(Some(tx));
                        loading_clone.set(false);
                    }
                    Err(e) => {
                        error_clone.set(Some(e.to_string()));
                        loading_clone.set(false);
                    }
                }
            });
        })
    };
    
    html! {
        <div>
            <h1 class="mb-4">{"Send Transaction"}</h1>
            
            if let Some(e) = (*error).clone() {
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {e}
                </div>
            }
            
            if let Some(tx) = (*success).clone() {
                <div class="alert alert-success" role="alert">
                    <i class="bi bi-check-circle me-2"></i>
                    {"Transaction sent successfully!"}
                    <div class="mt-2">
                        <strong>{"Transaction ID: "}</strong>
                        <span class="font-monospace">{tx.id}</span>
                    </div>
                </div>
            }
            
            <div class="card">
                <div class="card-body">
                    <form class="form-container">
                        <div class="mb-3">
                            <label for="sender" class="form-label">{"Sender Address"}</label>
                            <input 
                                type="text" 
                                class="form-control" 
                                id="sender" 
                                placeholder="0x..." 
                                value={(*sender).clone()}
                                onchange={on_sender_change}
                            />
                        </div>
                        
                        <div class="mb-3">
                            <label for="recipient" class="form-label">{"Recipient Address"}</label>
                            <input 
                                type="text" 
                                class="form-control" 
                                id="recipient" 
                                placeholder="0x..." 
                                value={(*recipient).clone()}
                                onchange={on_recipient_change}
                            />
                        </div>
                        
                        <div class="mb-3">
                            <label for="amount" class="form-label">{"Amount"}</label>
                            <input 
                                type="number" 
                                class="form-control" 
                                id="amount" 
                                placeholder="0" 
                                min="0"
                                value={(*amount).clone()}
                                onchange={on_amount_change}
                            />
                        </div>
                        
                        <div class="mb-3">
                            <label for="fee" class="form-label">{"Fee (optional)"}</label>
                            <input 
                                type="number" 
                                class="form-control" 
                                id="fee" 
                                placeholder="1" 
                                min="0"
                                value={(*fee).clone()}
                                onchange={on_fee_change}
                            />
                        </div>
                        
                        <div class="mb-3">
                            <label for="data" class="form-label">{"Data (optional)"}</label>
                            <textarea 
                                class="form-control" 
                                id="data" 
                                rows="3"
                                value={(*data).clone()}
                                onchange={on_data_change}
                            ></textarea>
                        </div>
                        
                        <button 
                            type="submit" 
                            class="btn btn-primary"
                            onclick={on_submit}
                            disabled={*loading}
                        >
                            if *loading {
                                <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                            }
                            {"Send Transaction"}
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}