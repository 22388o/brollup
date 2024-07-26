# Entries
`Bitcoin Virtual Machine` employs of six types of entries:

| Entry Type       |  Description                                                                                 |
|:-----------------|:---------------------------------------------------------------------------------------------|
| Transfer 💸      | Moves sats from a `Channel` into another `Channel`. Falls back to `Lift` outputs by default. |
| Call 📡          | Calls a smart contract. This may internally involve a `Transfer`.                            |
| Liftup ⬆️        | Onboards an account by turning a `Lift` output into a `VTXO`.                                |
| Liftdown ⬇️      | Offboards an account by swapping out `Channel` liquidity into a bare `Self` output.          |
| Recharge 🔋      | Refreshes a `VTXO` into a new `VTXO Projector`.                                              |
| Fallback 🌊      | Configures `Transfer` fallback preference to `VTXO` outputs, or back to `Lift` outputs.      |