# PS2DQ5-Sim

A simulator of PS2 Dragon Quest V

## Requirements
* Rust (Edition 2018)
    * Please install follow [the domumentation](https://www.rust-lang.org/tools/install)

## Usage
### Encount probabilty simulation

```sh
cargo run --release --example simulate_encount <encount_table_id> <iter_of_simuation>
```

For example, if you want to know the probability of encounter with `スライムナイト`  at around `ラインハット`, 

```sh
cargo run --release --example simulate_encount 22 100000
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/examples/simulate_encount 22 100000`
{
    Kind {
        id: 52,
        name: "アウルベアー",
    }: 0.20495,
    Kind {
        id: 38,
        name: "スライムナイト",
    }: 0.381,
    Kind {
        id: 32,
        name: "ダンスニードル",
    }: 0.26015,
    Kind {
        id: 58,
        name: "ドラゴンキッズ",
    }: 0.33635,
    Kind {
        id: 53,
        name: "イエティ",
    }: 0.33773,
}
```

so, it's `38.1%`.
