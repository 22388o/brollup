## Transaction Output Types
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