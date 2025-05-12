# PTA-Generator: Changelog

## Releases

### PTA-Generator release 25.05.1

#### New Features

* None

#### Changed Functionality

* Make proper shard structure with `month`

#### Fixes

* None


***

### PTA-Generator release 25.04.1

#### New Features

This is initial release, with features:

* Support for following PTA tools
  * [tackler](https://tackler.e257.fi/)
  * [ledger](https://ledger-cli.org/)
  * [hledger](https://hledger.org/)
  * [beancount](https://beancount.github.io/)
* Three major modes: 
  * `comm`: Journal with commodities
    * Tools: tackler, (h)ledger, beancount
  * `plain`: The simplest journal
    * Tools: tackler, (h)ledger
  * `audit`: Journal with transaction audit data  
    * Tools: tackler
* Three journal storage strategies:
  * `single`: Single journal
    * Tools: tackler, (h)ledger, beancount
  * `month`: Shard by transaction date
    * Tools: tackler
  * `txn`: Shard by transaction (each txn is in own file)
    * Tools: tackler

