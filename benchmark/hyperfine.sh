# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent

data_path=bench_data
set_size=1e5

hyperfine \
    --warmup 3 \
    --runs 5 \
    --command-name "set: $set_size - tackler"    "tackler --config $data_path/comm/set-$set_size-single.toml > /dev/null" \
    --command-name "set: $set_size - rustledger" "rledger report $data_path/comm/set-$set_size-single/txns/$set_size.beancount  balances > /dev/null" \
    --command-name "set: $set_size - ledger"     "ledger  -f $data_path/comm/set-$set_size-single/txns/$set_size.journal balance > /dev/null" \
    --command-name "set: $set_size - hledger"    "hledger -f $data_path/comm/set-$set_size-single/txns/$set_size.journal balance > /dev/null"

