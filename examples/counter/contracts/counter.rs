use rustorium_sdk::{Contract, State, Event};
use anyhow::Result;

#[derive(Event)]
pub struct CounterChanged {
    pub old_value: i32,
    pub new_value: i32,
    pub changed_by: Address,
}

#[derive(Contract)]
pub struct Counter {
    count: State<i32>,
}

#[contract]
impl Counter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            count: State::new(0),
        })
    }

    pub fn increment(&mut self) -> Result<()> {
        let old_value = *self.count;
        *self.count += 1;
        self.emit(CounterChanged {
            old_value,
            new_value: *self.count,
            changed_by: msg::sender(),
        })?;
        Ok(())
    }

    pub fn decrement(&mut self) -> Result<()> {
        let old_value = *self.count;
        *self.count -= 1;
        self.emit(CounterChanged {
            old_value,
            new_value: *self.count,
            changed_by: msg::sender(),
        })?;
        Ok(())
    }

    pub fn get_count(&self) -> Result<i32> {
        Ok(*self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustorium_sdk::testing::*;

    #[tokio::test]
    async fn test_counter() -> Result<()> {
        // テスト環境のセットアップ
        let env = TestEnv::new().await?;
        
        // コントラクトのデプロイ
        let counter = env.deploy_contract::<Counter>().await?;
        
        // 初期値の確認
        assert_eq!(counter.get_count().await?, 0);
        
        // インクリメントのテスト
        counter.increment().await?;
        assert_eq!(counter.get_count().await?, 1);
        
        // デクリメントのテスト
        counter.decrement().await?;
        assert_eq!(counter.get_count().await?, 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_events() -> Result<()> {
        // テスト環境のセットアップ
        let env = TestEnv::new().await?;
        let counter = env.deploy_contract::<Counter>().await?;
        
        // イベントの購読
        let mut events = counter.events().await?;
        
        // インクリメントの実行
        counter.increment().await?;
        
        // イベントの確認
        if let Some(event) = events.next().await {
            match event {
                CounterChanged { old_value, new_value, changed_by } => {
                    assert_eq!(old_value, 0);
                    assert_eq!(new_value, 1);
                    assert_eq!(changed_by, env.default_account());
                }
            }
        }
        
        Ok(())
    }
}
