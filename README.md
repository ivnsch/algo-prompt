# algo-prompt

![Continuous integration](https://github.com/i-schuetz/algo-prompt/actions/workflows/actions.yml/badge.svg)

Yew component to display Algorand transaction prompts.

![ScreenShot](screen/screen.png)

```rust
use algonaut::transaction::url::LinkableTransaction;
// ...

<AlgoPrompt transaction=some_transaction()></AlgoPrompt>
```
### Install
```
algo-prompt = { git = "https://github.com/i-schuetz/algo-prompt", branch = "main" }
```

### Run the example

```
cd example
trunk serve --port=3000
```

### Context
[Payment prompts with the Algorand mobile wallet](https://developer.algorand.org/articles/payment-prompts-with-algorand-mobile-wallet/)

[Algorand URI Scheme](https://developer.algorand.org/docs/reference/payment_prompts/)

### Contributing

1. Fork
2. Commit changes to a branch in your fork
3. Push your code and make a pull request
