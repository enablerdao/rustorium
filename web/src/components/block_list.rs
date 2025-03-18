use crate::models::Block;
use crate::utils::{format_address, format_block_hash, format_timestamp};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlockListProps {
    pub blocks: Vec<Block>,
    pub loading: bool,
}

#[function_component(BlockList)]
pub fn block_list(props: &BlockListProps) -> Html {
    if props.loading {
        return html! {
            <div class="d-flex justify-content-center my-5">
                <div class="spinner-border text-primary" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        };
    }
    
    if props.blocks.is_empty() {
        return html! {
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                {"No blocks found."}
            </div>
        };
    }
    
    html! {
        <div class="table-responsive">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th>{"Height"}</th>
                        <th>{"Hash"}</th>
                        <th>{"Timestamp"}</th>
                        <th>{"Validator"}</th>
                        <th>{"Transactions"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        props.blocks.iter().map(|block| {
                            html! {
                                <tr key={block.height}>
                                    <td>
                                        <a href={format!("/blocks/{}", block.height)} class="text-decoration-none">
                                            {block.height}
                                        </a>
                                    </td>
                                    <td class="text-monospace">{format_block_hash(&block.hash)}</td>
                                    <td>{format_timestamp(block.timestamp)}</td>
                                    <td>{format_address(&block.validator)}</td>
                                    <td>{block.transactions.len()}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}