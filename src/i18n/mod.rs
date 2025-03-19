use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LocaleConfig {
    pub language: String,
    messages: HashMap<String, String>,
}

impl LocaleConfig {
    pub fn new(language: &str) -> Self {
        let messages = match language {
            "ja" => {
                let mut m = HashMap::new();
                m.insert("welcome".to_string(), "Rustoriumへようこそ！".to_string());
                m.insert("select_action".to_string(), "実行したいアクションを選択してください：".to_string());
                m.insert("account".to_string(), "アカウント管理".to_string());
                m.insert("transaction".to_string(), "トランザクション".to_string());
                m.insert("smart_contract".to_string(), "スマートコントラクト".to_string());
                m.insert("blockchain".to_string(), "ブロックチェーン情報".to_string());
                m.insert("settings".to_string(), "設定".to_string());
                m.insert("exit".to_string(), "終了".to_string());
                m
            },
            "en" => {
                let mut m = HashMap::new();
                m.insert("welcome".to_string(), "Welcome to Rustorium!".to_string());
                m.insert("select_action".to_string(), "Select an action to perform:".to_string());
                m.insert("account".to_string(), "Account Management".to_string());
                m.insert("transaction".to_string(), "Transactions".to_string());
                m.insert("smart_contract".to_string(), "Smart Contracts".to_string());
                m.insert("blockchain".to_string(), "Blockchain Info".to_string());
                m.insert("settings".to_string(), "Settings".to_string());
                m.insert("exit".to_string(), "Exit".to_string());
                m
            },
            "zh" => {
                let mut m = HashMap::new();
                m.insert("welcome".to_string(), "欢迎使用 Rustorium！".to_string());
                m.insert("select_action".to_string(), "请选择要执行的操作：".to_string());
                m.insert("account".to_string(), "账户管理".to_string());
                m.insert("transaction".to_string(), "交易".to_string());
                m.insert("smart_contract".to_string(), "智能合约".to_string());
                m.insert("blockchain".to_string(), "区块链信息".to_string());
                m.insert("settings".to_string(), "设置".to_string());
                m.insert("exit".to_string(), "退出".to_string());
                m
            },
            "ko" => {
                let mut m = HashMap::new();
                m.insert("welcome".to_string(), "Rustorium에 오신 것을 환영합니다!".to_string());
                m.insert("select_action".to_string(), "실행할 작업을 선택하세요:".to_string());
                m.insert("account".to_string(), "계정 관리".to_string());
                m.insert("transaction".to_string(), "트랜잭션".to_string());
                m.insert("smart_contract".to_string(), "스마트 컨트랙트".to_string());
                m.insert("blockchain".to_string(), "블록체인 정보".to_string());
                m.insert("settings".to_string(), "설정".to_string());
                m.insert("exit".to_string(), "종료".to_string());
                m
            },
            _ => HashMap::new(),
        };

        Self {
            language: language.to_string(),
            messages,
        }
    }

    pub fn get_message<'a>(&'a self, key: &'a str) -> &'a str {
        self.messages.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}