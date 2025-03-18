use crate::models::AppPage;
use crate::pages::{
    accounts::AccountsPage, blocks::BlocksPage, dashboard::DashboardPage,
    send_transaction::SendTransactionPage, settings::SettingsPage, transactions::TransactionsPage,
};
use yew::prelude::*;

#[function_component(Layout)]
pub fn layout() -> Html {
    let current_page = use_state(|| AppPage::Dashboard);
    
    let on_nav_click = {
        let current_page = current_page.clone();
        Callback::from(move |page: AppPage| {
            current_page.set(page);
        })
    };
    
    html! {
        <div class="container-fluid">
            <div class="row">
                <div class="col-md-2 sidebar p-0">
                    <Sidebar current_page={(*current_page).clone()} on_nav_click={on_nav_click.clone()} />
                </div>
                <div class="col-md-10 main-content">
                    {
                        match *current_page {
                            AppPage::Dashboard => html! { <DashboardPage /> },
                            AppPage::Blocks => html! { <BlocksPage /> },
                            AppPage::Transactions => html! { <TransactionsPage /> },
                            AppPage::Accounts => html! { <AccountsPage /> },
                            AppPage::SendTransaction => html! { <SendTransactionPage /> },
                            AppPage::Settings => html! { <SettingsPage /> },
                        }
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub current_page: AppPage,
    pub on_nav_click: Callback<AppPage>,
}

#[function_component(Sidebar)]
fn sidebar(props: &SidebarProps) -> Html {
    let on_dashboard_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::Dashboard);
        })
    };
    
    let on_blocks_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::Blocks);
        })
    };
    
    let on_transactions_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::Transactions);
        })
    };
    
    let on_accounts_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::Accounts);
        })
    };
    
    let on_send_tx_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::SendTransaction);
        })
    };
    
    let on_settings_click = {
        let on_nav_click = props.on_nav_click.clone();
        Callback::from(move |_| {
            on_nav_click.emit(AppPage::Settings);
        })
    };
    
    html! {
        <div class="d-flex flex-column flex-shrink-0 p-3 sidebar">
            <a href="/" class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white text-decoration-none">
                <i class="bi bi-hdd-network me-2 fs-4"></i>
                <span class="fs-4">{"Rustorium"}</span>
            </a>
            <hr />
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::Dashboard).then_some("active"))} 
                        onclick={on_dashboard_click}
                    >
                        <i class="bi bi-speedometer2"></i>
                        {"Dashboard"}
                    </a>
                </li>
                <li>
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::Blocks).then_some("active"))} 
                        onclick={on_blocks_click}
                    >
                        <i class="bi bi-box"></i>
                        {"Blocks"}
                    </a>
                </li>
                <li>
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::Transactions).then_some("active"))} 
                        onclick={on_transactions_click}
                    >
                        <i class="bi bi-arrow-left-right"></i>
                        {"Transactions"}
                    </a>
                </li>
                <li>
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::Accounts).then_some("active"))} 
                        onclick={on_accounts_click}
                    >
                        <i class="bi bi-person-circle"></i>
                        {"Accounts"}
                    </a>
                </li>
                <li>
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::SendTransaction).then_some("active"))} 
                        onclick={on_send_tx_click}
                    >
                        <i class="bi bi-send"></i>
                        {"Send Transaction"}
                    </a>
                </li>
                <li>
                    <a 
                        class={classes!("nav-link", (props.current_page == AppPage::Settings).then_some("active"))} 
                        onclick={on_settings_click}
                    >
                        <i class="bi bi-gear"></i>
                        {"Settings"}
                    </a>
                </li>
            </ul>
            <hr />
            <div class="text-center text-white-50">
                <small>{"Rustorium v0.1.0"}</small>
            </div>
        </div>
    }
}