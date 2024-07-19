# Transaction Output Types
Bitcoin-VM employs 8 transaction output types (TXOs):
| Type         | Kind    |  Condition                                              |
|:-------------|:--------|:--------------------------------------------------------|
| Lift 🛗      | Bare    | `(Self + Operator) or (Self after 1 month)`             | 
| Projector 🎥 | Bare    | `(msg.senders + Operator) or (Operator after 3 months)` |
| Payload 📦   | Bare    | `(msg.senders after 1 day) or (Operator)`               |
| VTXO 💵      | Virtual | `(Self + Operator) or (Self after 3 months)`            |
| Connector 🔌 | Virtual | `(msg.sender + Operator)`                               |
| Channel 👥   | Virtual | `(Self + Operator) after degrading timelock`            |
| Self 👨‍💻      | Virtual | `Self`                                                  |
| Operator 🏭  | Virtual | `Operator`                                              |
## Lift 🛗
`Lift` is a bare, on-chain transaction output type used for onboarding (or boarding) to the Bitcoin VM.

## Projector 🎥
`Projector` is a bare, on-chain transaction output type contained in each pool transaction.  `Projector` is used for for projecting `VTXOs` and `Conenctors` in a pseudo-covenant manner.

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