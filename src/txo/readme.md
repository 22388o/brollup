# Transaction Output Types
Bitcoin Virtual Machine employs 10 transaction output (TXO) types:
| TXO Type               | Kind    |  Spending Condition                                                |
|:-----------------------|:--------|:----------------------------------------------------------|
| Lift 🛗                | Bare    | `(Self + Operator) or (Self after 1 month)`               | 
| Lift Connector 🔌      | Bare    | `Operator`                                                |
| VTXO 💵                | Virtual | `(Self + Operator) or (Self after 3 months)`              |
| VTXO Projector 🎥      | Bare    | `(msg.senders[] + Operator) or (Operator after 3 months)` |
| Channel 👥             | Virtual | `(Self + Operator) after degrading timelock`              |
| Channel Connector 🔌   | Virtual | `(msg.sender + Operator)`                                 |
| Connector Projector 🎥 | Bare    | `(msg.senders[] + Operator) or (Operator after 3 months)` |
| Payload 📦             | Bare    | `(msg.senders[] after 1 day) or (Operator)`               |
| Self 👨‍💻                | Virtual | `Self`                                                    |
| Operator 🏭            | Virtual | `Operator`                                                |
## Lift 🛗
`Lift` is a bare, on-chain transaction output type used for onboarding to the Bitcoin VM. When a `Lift` output is funded and has gained two on-chain confirmations, it can be swapped out for a 1:1 `VTXO` in a process known as lifting. In short, a `Lift` output lifts itself up to a `VTXO`.

#### Spending Condition
`Lift` carries two  spending conditions:
`(Self + Operator) or (Self after 1 month)`

-   Both `Self` and `Operator` must sign from the collaborative path `(Self + Operator)` to forfeit the `Lift` output in exchange for a 1:1 `VTXO`. `Self` swaps out the `Lift` output using the `Lift Connector` provided by the `Operator` to receive a new `VTXO` in return.
    
-   If the `Operator` is non-collaborative and does not sign from the collaborative path, `Self` can trigger the exit path `(Self after 1 month)` to reclaim their funds.

#### Swap Out

                        Prevouts                   Outs
                  ┌──────────────────┐     ┌──────────────────┐
               #0 │       Lift       │  #0 │     Operator     │
                  └──────────────────┘     └──────────────────┘
                  ┌──────────────────┐
               #1 │  Lift Connector  │
                  └──────────────────┘

## VTXO Projector 🎥
`Projector` is a bare, on-chain transaction output type contained in each pool transaction.  `Projector` is used for for projecting `VTXOs` and `Conenctors` in a pseudo-covenant manner.

                                                   ⋰ ┌──────────────────┐
                                                ⋰    │      VTXO #0     │
                                              ⋰      └──────────────────┘
                                            ⋰        ┌──────────────────┐
                    ┌──────────────────┐  ⋰          │      VTXO #1     │
                    │  VTXO Projector  │ ⋮            └──────────────────┘
                    └──────────────────┘  ⋱                  
                                            ⋱                 ┊
                                              ⋱     
                                                ⋱    ┌──────────────────┐
                                                  ⋱  │      VTXO #n     │
                                                    ⋱└──────────────────┘
                  

## Payload 📦
`Payload` is a bare, on-chain transaction output type contained in each pool transaction.  `Payload` stores entries, projector signatures, s commitments, and the fresh operator key of the session.

## VTXO 💵
`VTXO` is a virtual, off-chain transaction output type projected by the `Projector`.  `VTXO` contains funds of users.

## Connector 🔌
`Connector` is a virtual, off-chain transaction output type projected by the `Projector`.  `Connector` connects `VTXOs` into pool transactions.

## Channel 👥
`Channel` turns `VTXO` into a virtual channel, with a lifetime of 128 state transitions.

## Self 👨‍💻
`Self` is a virtual P2TR output containing the self inner-key with no script-path involved.

## Operator 🏭
`Operator` is a virtual P2TR output containing the operator inner-key with no script-path involved.