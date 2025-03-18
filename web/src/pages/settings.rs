use yew::prelude::*;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html! {
        <div>
            <h1 class="mb-4">{"Settings"}</h1>
            
            <div class="card mb-4">
                <div class="card-header">
                    <h5 class="mb-0">{"Node Configuration"}</h5>
                </div>
                <div class="card-body">
                    <div class="mb-3">
                        <label for="nodeId" class="form-label">{"Node ID"}</label>
                        <input type="text" class="form-control" id="nodeId" value="node-1" disabled={true} />
                    </div>
                    
                    <div class="mb-3">
                        <label for="dataDir" class="form-label">{"Data Directory"}</label>
                        <input type="text" class="form-control" id="dataDir" value="./data" disabled={true} />
                    </div>
                    
                    <div class="mb-3">
                        <label for="logLevel" class="form-label">{"Log Level"}</label>
                        <select class="form-select" id="logLevel">
                            <option value="trace">{"Trace"}</option>
                            <option value="debug">{"Debug"}</option>
                            <option value="info" selected={true}>{"Info"}</option>
                            <option value="warn">{"Warn"}</option>
                            <option value="error">{"Error"}</option>
                        </select>
                    </div>
                </div>
            </div>
            
            <div class="card mb-4">
                <div class="card-header">
                    <h5 class="mb-0">{"Network Configuration"}</h5>
                </div>
                <div class="card-body">
                    <div class="mb-3">
                        <label for="listenAddr" class="form-label">{"Listen Address"}</label>
                        <input type="text" class="form-control" id="listenAddr" value="0.0.0.0" disabled={true} />
                    </div>
                    
                    <div class="mb-3">
                        <label for="listenPort" class="form-label">{"Listen Port"}</label>
                        <input type="number" class="form-control" id="listenPort" value="30333" disabled={true} />
                    </div>
                    
                    <div class="mb-3">
                        <label for="maxPeers" class="form-label">{"Maximum Peers"}</label>
                        <input type="number" class="form-control" id="maxPeers" value="50" />
                    </div>
                </div>
            </div>
            
            <div class="card mb-4">
                <div class="card-header">
                    <h5 class="mb-0">{"Consensus Configuration"}</h5>
                </div>
                <div class="card-body">
                    <div class="mb-3">
                        <label for="algorithm" class="form-label">{"Consensus Algorithm"}</label>
                        <select class="form-select" id="algorithm" disabled={true}>
                            <option value="avalanche" selected={true}>{"Avalanche"}</option>
                        </select>
                    </div>
                    
                    <div class="mb-3">
                        <label for="blockTime" class="form-label">{"Block Time (ms)"}</label>
                        <input type="number" class="form-control" id="blockTime" value="2000" />
                    </div>
                </div>
            </div>
            
            <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                <button class="btn btn-primary" type="button">
                    <i class="bi bi-save me-2"></i>
                    {"Save Settings"}
                </button>
            </div>
        </div>
    }
}