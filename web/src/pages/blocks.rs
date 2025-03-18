use crate::api::ApiClient;
use crate::components::block_list::BlockList;
use crate::models::Block;
use yew::prelude::*;

#[function_component(BlocksPage)]
pub fn blocks_page() -> Html {
    let blocks = use_state(|| Vec::<Block>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let current_page = use_state(|| 0u64);
    let has_more = use_state(|| true);
    
    // Load blocks
    {
        let blocks = blocks.clone();
        let loading = loading.clone();
        let error = error.clone();
        let current_page = current_page.clone();
        let has_more = has_more.clone();
        
        use_effect_with_deps(
            move |_| {
                loading.set(true);
                
                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get_blocks(None, Some(10)).await {
                        Ok(new_blocks) => {
                            blocks.set(new_blocks.clone());
                            has_more.set(new_blocks.len() >= 10);
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
            *current_page,
        );
    }
    
    let on_prev_page = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            if *current_page > 0 {
                current_page.set(*current_page - 1);
            }
        })
    };
    
    let on_next_page = {
        let current_page = current_page.clone();
        let has_more = has_more.clone();
        Callback::from(move |_| {
            if *has_more {
                current_page.set(*current_page + 1);
            }
        })
    };
    
    html! {
        <div>
            <h1 class="mb-4">{"Blocks"}</h1>
            
            if let Some(e) = (*error).clone() {
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    {e}
                </div>
            }
            
            <div class="card">
                <div class="card-body">
                    <BlockList blocks={(*blocks).clone()} loading={*loading} />
                    
                    <div class="d-flex justify-content-between mt-3">
                        <button 
                            class="btn btn-outline-primary" 
                            disabled={*current_page == 0}
                            onclick={on_prev_page}
                        >
                            <i class="bi bi-chevron-left me-1"></i>
                            {"Previous"}
                        </button>
                        
                        <button 
                            class="btn btn-outline-primary" 
                            disabled={!*has_more}
                            onclick={on_next_page}
                        >
                            {"Next"}
                            <i class="bi bi-chevron-right ms-1"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}