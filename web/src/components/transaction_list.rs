use crate::models::Transaction;
use crate::utils::{format_address, format_amount, format_timestamp, format_tx_id};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TransactionListProps {
    pub transactions: Vec<Transaction>,
    pub loading: bool,
}

#[function_component(TransactionList)]
pub fn transaction_list(props: &TransactionListProps) -> Html {
    if props.loading {
        return html! {
            <div class="d-flex justify-content-center my-5">
                <div class="spinner-border text-primary" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        };
    }
    
    if props.transactions.is_empty() {
        return html! {
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                {"No transactions found."}
            </div>
        };
    }
    
    html! {
        <div class="list-group">
            {
                props.transactions.iter().map(|tx| {
                    html! {
                        <div class="list-group-item transaction-item" key={tx.id.clone()}>
                            <div class="d-flex justify-content-between align-items-center">
                                <div>
                                    <div class="transaction-id">
                                        <a href={format!("/transactions/{}", tx.id)} class="text-decoration-none">
                                            {format_tx_id(&tx.id)}
                                        </a>
                                    </div>
                                    <div class="d-flex mt-1">
                                        <div class="me-2">
                                            <small class="text-muted">{"From:"}</small>
                                            <span class="address ms-1">{format_address(&tx.sender)}</span>
                                        </div>
                                        <div class="me-2">
                                            <i class="bi bi-arrow-right text-muted"></i>
                                        </div>
                                        <div>
                                            <small class="text-muted">{"To:"}</small>
                                            <span class="address ms-1">{format_address(&tx.recipient)}</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="text-end">
                                    <div class="amount">{format_amount(tx.amount)}</div>
                                    <div class="timestamp">{format_timestamp(tx.timestamp)}</div>
                                </div>
                            </div>
                            <div class="mt-2">
                                <span class={classes!(
                                    "badge",
                                    match tx.status.as_str() {
                                        "Confirmed" => "bg-success",
                                        "Pending" => "bg-warning",
                                        "Failed" => "bg-danger",
                                        _ => "bg-secondary"
                                    }
                                )}>
                                    {&tx.status}
                                </span>
                                <span class="badge bg-secondary ms-1">
                                    {"Fee: "}{tx.fee}
                                </span>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}