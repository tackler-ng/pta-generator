# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent

data_path=bench_data
set_size=1e4


hyperfine \
    --warmup 3 \
    --runs 10 \
    --command-name 'tackler' "tackler --config $data_path/comm/set-$set_size-single.toml > /dev/null" \
    --command-name 'ledger'  "ledger -f $data_path/comm/set-$set_size-single/txns/$set_size.journal balance > /dev/null" \
    --command-name 'rustledger' "rledger report $data_path/comm/set-$set_size-single/txns/$set_size.beancount  balances > /dev/null" \
    --command-name 'hledger' "hledger -f $data_path/comm/set-$set_size-single/txns/$set_size.journal balance > /dev/null"

