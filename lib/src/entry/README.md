# Entries
`Bitcoin Virtual Machine` employs of six types of entries.

| Entry Type       |  Description                                                               |
|:-----------------|:---------------------------------------------------------------------------|
| Transfer 💸      | Moves sats from an account to another.                                     |
| Call 📡          | Calls a smart contract. This may internally involve a `Transfer`.          |
| Liftup ⬆️        | Onboards an account by turning a `Lift` output into a `VTXO`.              |
| Liftdown ⬇️      | Offboards an account by turning a `VTXO` output into a bare `Self` output. |
| Recharge 🔋      | Refreshes a `VTXO` to be contained in a new `VTXO Projector`.              |
| Reserved 📁      | Does nothing. Reserved for future upgrade.                                 |