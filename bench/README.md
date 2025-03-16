# Benchmarks

![Plot](plot.png)

We use a simple Python script with `uv` to plot the benchmark results above.

```sh
uv run plot.py
```

We have 4 bars here:

- **bun**: using Bun runtime for SnarkJS, multi-threading disabled due to [snarkjs#490](https://github.com/iden3/snarkjs/pull/490)
- **bun (ffi: arkworks)**: using Bun runtime with `bun:ffi` to call Arkworks
- **node**: using Node runtime for SnarkJS, with multi-threading
- **node (ffi: arkworks)**: using Node runtime with `ffi-rs` to call Arkworks

The results show that using FFI does not help with performance w.r.t Node runtime, unless the circuit is very small (e.g. $\lt 3000$ constraints). Using Bun, it is always better to use the FFI library, notably because the SnarkJS prover in Bun works in a single thread, unlike its NodeJS counter-part that runs multi-threaded. Using FFI saves some time there, at the scale of x2 as the circuit size grows.

In short, using FFI has a performance increase when using Bun, and moreover, this project shows the proof-of-concept to easily use out-of-TS prover backends using the interface that we have built for Arkworks.

> [!NOTE]
>
> Lambdaworks prover is also added, but it does its own trusted setup & its proof is not compatible with SnarkJS yet.

## Settings

Benchmark settings:

- Using a Macbook Air M3
- Bun v1.2.5.
- NodeJS v22.13.1
- 1 warm-up with proof verification for sanity check
- 5 iterations for each case, average reported

All benchmarks work over a computed raw witness file, and the proof is checked to be verified via SnarkJS as well.

## Raw Outputs

The raw results for Bun runtime are shown below:

```sh
Using FFI library at /Users/erhant/circomkit-ffi/example/src/libcircomkit_ffi-macOS-arm64.dylib for Bun
Doing warm-up iterations...

Proving for multiplier_3
Arkworks average time (5 runs): 2.08ms
SnarkJS average time (5 runs): 91.00ms

Proving for multiplier_30
Arkworks average time (5 runs): 4.76ms
SnarkJS average time (5 runs): 97.81ms

Proving for multiplier_300
Arkworks average time (5 runs): 30.45ms
SnarkJS average time (5 runs): 186.71ms

Proving for multiplier_3000
Arkworks average time (5 runs): 280.94ms
SnarkJS average time (5 runs): 697.93ms

Proving for multiplier_30000
Arkworks average time (5 runs): 2812.81ms
SnarkJS average time (5 runs): 4491.08ms

Proving for multiplier_300000
Arkworks average time (5 runs): 28472.77ms
SnarkJS average time (5 runs): 46463.74ms
```

The raw results for NodeJS runtime are shown below:

```sh
Using FFI library at /Users/erhant/circomkit-ffi/example/src/libcircomkit_ffi-macOS-arm64.dylib for Node
Doing warm-up iterations...

Proving for multiplier_3
Arkworks average time (5 runs): 2.10ms
SnarkJS average time (5 runs): 17.82ms

Proving for multiplier_30
Arkworks average time (5 runs): 4.77ms
SnarkJS average time (5 runs): 12.62ms

Proving for multiplier_300
Arkworks average time (5 runs): 36.10ms
SnarkJS average time (5 runs): 30.70ms

Proving for multiplier_3000
Arkworks average time (5 runs): 275.04ms
SnarkJS average time (5 runs): 148.49ms

Proving for multiplier_30000
Arkworks average time (5 runs): 2826.17ms
SnarkJS average time (5 runs): 1126.89ms

Proving for multiplier_300000
Arkworks average time (5 runs): 28184.95ms
SnarkJS average time (5 runs): 27709.03ms
```
