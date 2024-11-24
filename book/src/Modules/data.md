# `data`

The `data` module encompasses everything data related. 

That is, anything that can be observed, either in markets or derived from market observable data, and also facilities to manage that data. 

Another form of data is contextual (or reference) data. These are things such as calendars and date conventions. While there are facilities to handle these data inside the `data` module, the underlying implementations are in other modules, such as the `time` module.

## Curves

Curves can be fit to market data. Here we include an example of a spot curve being fitted.

![`Spot curve`](../assets/spotcurve.png)

```rust
{{#include ../../../examples/examples/curves_spot.rs}}
```
