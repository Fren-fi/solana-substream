# solana-substreams
Solana substreams monorepo.

## Getting started

Try out a module directly from the command line with `substreams`:

```bash
# System Program
substreams gui frens-events
# SPL Token
substream gui spl-token-events
# Raydium AMM
substreams gui raydium-amm-events
# MPL Token Metadata
substreams gui mpl-token-metadata-events
```

You can access the substreams in this repo either by specifying them as a dependency through `substreams.yaml`, or by using them as libraries (see setup).