# stock-prices

This is a script that given a list of stock symbols returns their current
price in GBP [using the beancount syntax](https://beancount.github.io/docs/fetching_prices_in_beancount.html).

The format for the input file is as follows:

```
AAPL
META
AMZN
BRK-B BRK.B # first is Yahoo symbol, second is beancount symbol to emit
```
