### To reproduce the results in the table

1. [Install](https://www.rust-lang.org/tools/install) Rust toolchain.
2. Run one of the commands bellow.

<br/>

#### Basic Accountable Scheme 
> cargo test --release --features "parallel print-trace" --test basic 10

> cargo test --release --features "parallel print-trace" --test basic 16 

> cargo test --release --features "parallel print-trace" --test basic 20
 
<br/>

#### Packed Accountable Scheme
> cargo test --release --features "parallel print-trace" --test packed 10

> cargo test --release --features "parallel print-trace" --test packed 16

> cargo test --release --features "parallel print-trace" --test packed 20

<br/>

#### Counting Scheme
> cargo test --release --features "parallel print-trace" --test counting 10

> cargo test --release --features "parallel print-trace" --test counting 16

> cargo test --release --features "parallel print-trace" --test counting 20

<br/>

The output should look, for example, like
```
Running test for the 'basic' scheme for N = 2^10
Start:   Prover
End:     Prover ....................................................................520.741ms
Start:   Verifier
End:     Verifier ..................................................................25.871ms
```