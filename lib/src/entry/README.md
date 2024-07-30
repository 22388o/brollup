# Entries
`Bitcoin Virtual Machine` employs of 6 types of entries:

| Entry Type       |  Description                                                                           |
|:-----------------|:---------------------------------------------------------------------------------------|
| Transfer 💸      | Moves sats from a `Channel` into another `Channel`. Falls back to `Lift` if necessary. |
| Call 📡          | Calls a smart contract. This may internally involve `Transfer`.                        |
| Liftup ⬆️        | Turns `Lift` into a `VTXO`.                                                            |
| Liftdown ⬇️      | Swaps out `Channel` liquidity into a bare `Self`.                                      |
| Recharge 🔋      | Refreshes `Channel` liquidity into a new `VTXO`.                                       |
| Reserved 📁      | Fails the entry. Reserved for future upgrades.                                         |