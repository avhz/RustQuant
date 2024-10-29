# Options

## European Vanilla Options

To price a vanilla European option, we begin by defining the option itself:

```rust,noplayground
{{#include ../../../../examples/examples/option_pricing_vanilla.rs:option_definition}}
```

Then we need to define a model to price the option with:

```rust,noplayground
{{#include ../../../../examples/examples/option_pricing_vanilla.rs:model_definitions}}
```

Lastly, we construct an `AnalyticOptionPricer`, from which we can generate a report.
The report simply prints the option, model, price, and Greeks.

```rust,noplayground
{{#include ../../../../examples/examples/option_pricing_vanilla.rs:option_pricing}}
```