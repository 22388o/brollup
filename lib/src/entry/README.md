# Entries
`Bitcoin Virtual Machine` employs of 6 types of entries:

| Entry Type       |  Description                                                                                 |
|:-----------------|:---------------------------------------------------------------------------------------------|
| Transfer 💸      | Transfers sats from one `Channel` to another. Defaults to `Lift` if liquidity is insufficient.|
| Call 📡          | Calls a smart contract. This may internally involve `Transfer`.                               |
| Liftup ⬆️        | Turns `Lift` into `VTXO`.                                                                     |
| Liftdown ⬇️      | Swaps out `Channel` liquidity into bare `Self`.                                               |
| Recharge 🔋      | Refreshes `Channel` liquidity into fresh, new `VTXO`.                                         |
| Reserved 📁      | Fails the entry. Reserved for future upgrades.                                                |