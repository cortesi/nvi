//! Functions for managing plugin demo functionality.

use crate::error::Result;
use crate::{client::Client, NviPlugin};

/// A function that can be registered with the Demo struct.
pub type DemoFunction = Box<
    dyn Fn(&crate::client::Client) -> futures::future::BoxFuture<'static, crate::error::Result<()>>,
>;

/// Holds a collection of named demo functions that can be executed with a Client.
#[derive(Default)]
pub struct Demos {
    functions: std::collections::HashMap<String, DemoFunction>,
}

impl Demos {
    /// Creates a new Demo instance.
    pub fn new() -> Self {
        Self {
            functions: std::collections::HashMap::new(),
        }
    }

    /// Returns an alphabetically sorted list of available demos.
    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// Adds a named function to the demo collection.
    pub fn add<F, Fut>(&mut self, name: impl Into<String>, f: F)
    where
        F: Fn(&Client) -> Fut + 'static,
        Fut: futures::Future<Output = Result<()>> + Send + 'static,
    {
        self.functions
            .insert(name.into(), Box::new(move |client| Box::pin(f(client))));
    }

    /// Run a named demo function with a plugin instance.
    ///
    /// This starts a new Neovim instance, connects the plugin to it, runs the demo,
    /// and then shuts everything down.
    pub async fn run<T>(&self, name: &str, plugin: T) -> Result<()>
    where
        T: NviPlugin + Send + Sync + Unpin + 'static,
    {
        let t = crate::test::NviTest::builder().run(plugin).await?;
        let f = self
            .functions
            .get(name)
            .ok_or_else(|| crate::error::Error::Plugin(format!("no such demo: {}", name)))?;
        f(&t.client).await?;
        t.finish().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NviPlugin;

    #[derive(Default)]
    struct TestPlugin;
    impl NviPlugin for TestPlugin {
        fn name(&self) -> String {
            "test".into()
        }
    }

    #[tokio::test]
    async fn test_demos() {
        let mut d = Demos::new();
        assert!(d.list().is_empty());

        d.add("two", |c| {
            let c = c.clone();
            async move {
                c.lua("print('demo two')").await?;
                Ok(())
            }
        });
        d.add("one", |c| {
            let c = c.clone();
            async move {
                c.lua("print('demo one')").await?;
                Ok(())
            }
        });

        let lst = d.list();
        assert_eq!(lst, vec!["one", "two"]);

        // Test actually running a demo
        d.run("one", TestPlugin).await.unwrap();
    }
}
