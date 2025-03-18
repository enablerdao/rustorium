use crate::api::ApiClient;
use crate::models::Account;
use crate::utils::{format_address, format_amount};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(AccountsPage)]
pub fn accounts_page() -> Html {
    let account = use_state(|| None::<Account>);
    let address_input = use_state(|| String::new());
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    
    let on_address_change = {
        let address_input = address_input.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            address_input.set(input.value());
        })
    };
    
    let on_search = {
        let address_input = address_input.clone();
        let account = account.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            
            let address = (*address_input).clone();
            if address.is_empty() {
                error.set(Some("Please enter an address".to_string()));
                return;
            }
            
            loading.set(true);
            error.set(None);
            
            let account_clone = account.clone();
            let loading_clone = loading.clone();
            let error_clone = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get_account(&address).await {
                    Ok(new_account) => {
                        account_clone.set(Some(new_account));
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
            <h1 class="mb-4">{"Accounts"}</h1>
            
            <div class="card mb-4">
                <div class="card-body">
                    <form>
                        <div class="input-group">
                            <input 
                                type="text" 
                                class="form-control" 
                                placeholder="Enter account address (0x...)" 
                                value={(*address_input).clone()}
                                onchange={on_address_change}
                            />
                            <button 
                                class="btn btn-primary" 
                                type="submit" 
                                onclick={on_search}
                                disabled={*loading}
                            >
                                if *loading {
                                    <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                }
                                {"Search"}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
            
            if let Some(e) = (*error).clone() {
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {e}
                </div>
            }
            
            if let Some(acc) = (*account).clone() {
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">{"Account Details"}</h5>
                    </div>
                    <div class="card-body">
                        <div class="row mb-3">
                            <div class="col-md-3 fw-bold">{"Address:"}</div>
                            <div class="col-md-9 font-monospace">{acc.address}</div>
                        </div>
                        <div class="row mb-3">
                            <div class="col-md-3 fw-bold">{"Balance:"}</div>
                            <div class="col-md-9">{format_amount(acc.balance)}</div>
                        </div>
                        <div class="row mb-3">
                            <div class="col-md-3 fw-bold">{"Nonce:"}</div>
                            <div class="col-md-9">{acc.nonce}</div>
                        </div>
                        <div class="row">
                            <div class="col-md-3 fw-bold">{"Contract:"}</div>
                            <div class="col-md-9">
                                if acc.is_contract {
                                    <span class="badge bg-success">{"Yes"}</span>
                                } else {
                                    <span class="badge bg-secondary">{"No"}</span>
                                }
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}