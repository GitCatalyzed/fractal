STORE CONTRACT
osmosisd tx wasm store /workspaces/osmosis/cosmwasm/artifacts/core_payment.wasm --from lo-test2 --chain-id localosmosis --gas auto --gas-adjustment 1.3 -b block --keyring-backend test --fees 4500uosmo

INSTANTIATE CONTRACT
osmosisd tx wasm instantiate [X] '{}' --from lo-test1 --keyring-backend test --chain-id localosmosis --gas auto --fees 4500uosmo --gas-adjustment 1.3 -b block --label isolated_invoice --no-admin

CREATE INVOICE:
osmosisd tx wasm execute osmo1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dqptxsjf '{"create_invoice": {"payer_addr":"osmo18s5lynnmx37hq4wlrw9gdn68sg2uxp5rgk26vv","payer_alias":"Shaw Steel", "invoice_id":"I0000001", "invoiced_value":"35000.1","date_due":"09/01/2023", "pay_unit":"USD", "receipt_unit":"USD"}}' --from lo-test1 --keyring-backend test --fees 875uosmo -b block --chain-id localosmosis

PAY INVOICE:
osmosisd tx wasm execute osmo17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs5yczr8 '{"pay_invoice":  {"invoice_id":"I0000001","payer_alias":"Shaw Steel", "payment_amount":"5000", "pay_unit":"USD"}}' --from lo-test2 --keyring-backend test --fees 875uosmo -b block --chain-id localosmosis

QUERY ALL INVOICES FOR CONTRACT ADDRESS:
osmosisd query wasm contract-state smart osmo1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dqptxsjf '{"all_invoices":{}}'




# Osmosis

![Banner!](assets/banner.png)

[![Project Status: Active -- The project has reached a stable, usable
state and is being actively
developed.](https://img.shields.io/badge/repo%20status-Active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![GoDoc](https://img.shields.io/badge/godoc-reference-blue?style=flat-square&logo=go)](https://pkg.go.dev/github.com/osmosis-labs/osmosis/v11)
[![Go Report
Card](https://goreportcard.com/badge/github.com/osmosis-labs/osmosis?style=flat-square)](https://goreportcard.com/report/github.com/osmosis-labs/osmosis/v11)
[![Version](https://img.shields.io/github/tag/osmosis-labs/osmosis.svg?style=flat-square)](https://github.com/osmosis-labs/osmosis/releases/latest)
[![License:
Apache-2.0](https://img.shields.io/github/license/osmosis-labs/osmosis.svg?style=flat-square)](https://github.com/osmosis-labs/osmosis/blob/main/LICENSE)
[![Lines Of
Code](https://img.shields.io/tokei/lines/github/osmosis-labs/osmosis?style=flat-square)](https://github.com/osmosis-labs/osmosis)
[![GitHub
Super-Linter](https://img.shields.io/github/actions/workflow/status/osmosis-labs/osmosis/lint.yml?style=flat-square&label=Lint)](https://github.com/marketplace/actions/super-linter)
[![Discord](https://badgen.net/badge/icon/discord?icon=discord&label)](https://discord.gg/osmosis)

Osmosis is a fair-launched, customizable automated market maker for
interchain assets that allows the creation and management of
non-custodial, self-balancing, interchain token index similar to one of
Balancer.

Inspired by [Balancer](http://balancer.finance/whitepaper) and Sunny
Aggarwal's '[DAOfying Uniswap Automated Market Maker
Pools](https://www.sunnya97.com/blog/daoifying-uniswap-automated-market-maker-pools)',
the goal for Osmosis is to provide the best-in-class tools that extend
the use of AMMs within the Cosmos ecosystem beyond traditional token
swap-type use cases. Bonding curves, while have found its primary use
case in decentralized exchange mechanisms, its potential use case can be
further extended through the customizability that Osmosis offers.
Through the customizability offered by Osmosis such as custom-curve AMMs,
dynamic adjustments of spread factors, multi-token liquidity pools--the AMM
can offer decentralized formation of token fundraisers, interchain
staking, options market, and more for the Cosmos ecosystem.

Whereas most Cosmos zones have focused their incentive scheme on the
delegators, Osmosis attempts to align the interests of multiple
stakeholders of the ecosystem such as LPs, DAO members, as well as
delegators. One mechanism that is introduced is how staked liquidity
providers have sovereign ownership over their pools, and through the
pool governance process allow them to adjust the parameters depending on
the pool's competition and market conditions. Osmosis is a sovereign
Cosmos zone that derives its sovereignty not only from its
application-specific blockchain architecture but also the collective
sovereignty of the LPs that has aligned interest to different tokens
that they are providing liquidity for.

## System Requirements

This system spec has been tested by many users and validators and found
to be comfortable:

- Quad Core or larger AMD or Intel (amd64) CPU
  - ARM CPUs like the Apple M1 are not supported at this time.
- 64GB RAM (A lot can be in swap)
- 1TB NVMe Storage
- 100MBPS bidirectional internet connection

You can run Osmosis on lower-spec hardware for each component, but you
may find that it is not highly performant or prone to crashing.

## Documentation

For the most up to date documentation please visit
[docs.osmosis.zone](https://docs.osmosis.zone/)

## Joining the Mainnet

[Please visit the official instructions on how to join the Mainnet
here.](https://docs.osmosis.zone/networks/join-mainnet)

Thank you for supporting a healthy blockchain network and community by
running an Osmosis node!

## Contributing

The contributing guide for Osmosis explains the branching structure, how
to use the SDK fork, and how to make / test updates to SDK branches.

## LocalOsmosis

LocalOsmosis is a containerized local Osmosis testnet used for trying out new features locally. 
LocalOsmosis documentation can be found [here](https://github.com/osmosis-labs/osmosis/tree/main/tests/localosmosis)
