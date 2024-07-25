# Entries
`Bitcoin Virtual Machine` employs of six types of entries.

| Entry Type       |  Description                                     |
|:-----------------|:-------------------------------------------------|
| Transfer 💸      | Transfers sats.                                  |
| Call 📡          | Calls a smart contracts.                         |
| Liftup ⬆️        | Turns a `Lift` output into a `VTXO`.             |
| Liftdown ⬇️      | Turns a `VTXO` output into a bare `Self` output. |
| Recharge 🔋      | Refreshes a `VTXO` into a new `VTXO Projector`.  |
| Reserved 📁      | Fails. Reserved for a future upgrade.            |