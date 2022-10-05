# Sqlite insert benchmark
Quick & dirty test of how quickly sqlite will accept rows

Uses insert batching & an in-memory database

## Running locally
1. Clone this repo
2. `cargo run`

## Results
```
Tested on M1 Macbook Pro
Batch size : rows per second
50    : 1_290_988
100   : 1_310_272 <-- peak
250   : 1_221_597
500   : 1_182_732
1_000 : 1_162_250
10_000: 1_110_494
50_000:  867_603
```
