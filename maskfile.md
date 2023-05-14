# Tasks for Gossip Glomers

## build

**OPTIONS**
* bin
    * flags: --bin
    * type: string
    * desc: Print path to specified target bin

~~~sh
cargo build ${bin:+--bin $bin}
~~~

## get-bin
> Prints binary path to stdout. In case of multiple binaries prints "first" according to cargo metadata or the one specified by `--bin` option

**OPTIONS**
* build
    * flags: -b --build
    * desc: Run build before printing path
* bin
    * flags: --bin
    * type: string
    * desc: Print path to specified target bin

~~~bash
[[ "$build" == "true" ]] && $MASK build ${bin:+--bin $bin} >&2

if [[ -n "$bin" ]]; then
    cargo metadata --no-deps --format-version 1 | jq -r ".target_directory + \"/debug/$bin\""
else
    cargo metadata --no-deps --format-version 1 | jq -r '.target_directory + "/debug/" + (.packages[0].targets | map(select(.kind | any(. == "bin"))))[0].name'
fi
~~~

## test
> Runs custom set of tests

~~~sh
cargo test
~~~

## gg_test
> Runs Gossip Glomers test suites

### 1
> Runs Gossip Glomers test `echo`

**OPTIONS**
* maelstrom
    * flags: -m --maelstrom
    * type: string
    * desc: Path to binary of Maelstrom
* bin
    * flags: --bin
    * type: string
    * desc: Path to binary to test (defaults to default binary from `get-bin`)

~~~bash
m_bin="${maelstrom:-../maelstrom/maelstrom}"
t_bin="${bin:-$($MASK get-bin --build)}"
"$m_bin" test -w echo --bin "$t_bin" --node-count 1 --time-limit 10
~~~

### 2
> Runs Gossip Glomers `unique-ids`

**OPTIONS**
* maelstrom
    * flags: -m --maelstrom
    * type: string
    * desc: Path to binary of Maelstrom
* bin
    * flags: --bin
    * type: string
    * desc: Path to binary to test (defaults to default binary from `get-bin`)

~~~bash
m_bin="${maelstrom:-../maelstrom/maelstrom}"
t_bin="${bin:-$($MASK get-bin --build)}"
"$m_bin" test -w unique-ids --bin "$t_bin" --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
~~~

### 3a
> Runs Gossip Glomers `broadcast`

**OPTIONS**
* maelstrom
    * flags: -m --maelstrom
    * type: string
    * desc: Path to binary of Maelstrom
* bin
    * flags: --bin
    * type: string
    * desc: Path to binary to test (defaults to default binary from `get-bin`)

~~~bash
m_bin="${maelstrom:-../maelstrom/maelstrom}"
t_bin="${bin:-$($MASK get-bin --build)}"
"$m_bin" test -w broadcast --bin "$t_bin" --node-count 1 --time-limit 20 --rate 10
~~~

### 3b
> Runs Gossip Glomers `broadcast` 3b

**OPTIONS**
* maelstrom
    * flags: -m --maelstrom
    * type: string
    * desc: Path to binary of Maelstrom
* bin
    * flags: --bin
    * type: string
    * desc: Path to binary to test (defaults to default binary from `get-bin`)

~~~bash
m_bin="${maelstrom:-../maelstrom/maelstrom}"
t_bin="${bin:-$($MASK get-bin --build)}"
"$m_bin" test -w broadcast --bin "$t_bin" --node-count 5 --time-limit 20 --rate 10
~~~
