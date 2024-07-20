# Transaction Outputs
`Bitcoin Virtual Machine` employs of ten types of transaction outputs (TXOs):

| TXO Type               | Kind    |  Spending Condition                                       |
|:-----------------------|:--------|:----------------------------------------------------------|
| Lift 🛗                | Bare    | `(Self + Operator) or (Self after 1 month)`               | 
| Bare Connector 🔌      | Bare    | `Operator`                                                |
| VTXO 💵                | Virtual | `(Self + Operator) or (Self after 3 months)`              |
| VTXO Projector 🎥      | Bare    | `(msg.senders[] + Operator) or (Operator after 3 months)` |
| Channel 👥             | Virtual | `(Self + Operator) after degrading timelock`              |
| Virtual Connector 🔌   | Virtual | `(msg.sender + Operator)`                                 |
| Connector Projector 🎥 | Bare    | `(msg.senders[] + Operator) or (Operator after 3 months)` |
| Payload 📦             | Bare    | `(msg.senders[] after 1 day) or (Operator)`               |
| Self 👨‍💻                | Virtual | `Self`                                                    |
| Operator 🏭            | Virtual | `Operator`                                                |

Five of the transaction output types are bare, meaning they are literal, on-chain transaction outputs that consume block space, while the other five are virtual, meaning they are committed but not yet revealed transaction outputs that optimistically consume no block space.

The `Bitcoin Virtual Machine` advances the rollup state by chaining `Pool Transactions` at regular intervals. Three output types—`VTXO Projector`, `Connector Projector`, and `Payload`—and optionally one or more `Bare Connectors` are contained in the `Pool Transaction`.

                                                                             ⋰
                                                                           ⋰  ┌────────────────┐   ┌────────────────┐
                                                                         ⋰    │     VTXO #0    │-->│   Channel #0   │ 
                                                                       ⋰      └────────────────┘   └────────────────┘
                                                                     ⋰                 ┊                   ┊
                                                                   ⋰          ┌────────────────┐   ┌────────────────┐
                 Prevouts                      Outs              ⋰            │     VTXO #z    │-->│   Channel #z   │ 
          ┌───────────────────┐      ┌─────────────────────┐   ⋰              └────────────────┘   └────────────────┘
       #0 │    Prev Payload   │   #0 │    VTXO Projector   │ 🎥 ┈ ┈ ┈ ┈ ┈ ┈ ┈ ┈      
          └───────────────────┘      └─────────────────────┘         
                    ┊                ┌─────────────────────┐                          
          ┌───────────────────┐   #1 │ Connector Projector │ 🎥 ┈ ┈ ┈ ┈ ┈ ┈ ┈ ┈            
       #n │  Other Prevouts   │      └─────────────────────┘   ⋱              ┌────────────────────────┐  
          └───────────────────┘      ┌─────────────────────┐     ⋱            │  Virtual Connector #0  │       
                                  #2 │       Payload       │       ⋱          └────────────────────────┘
                                     └─────────────────────┘         ⋱                     ┊
                                     ┌─────────────────────┐           ⋱      ┌────────────────────────┐   
                                  #3 │  Bare Connector #0  │             ⋱    │  Virtual Connector #y  │
                                     └─────────────────────┘               ⋱  └────────────────────────┘
                                                ┊                            ⋱
                                     ┌─────────────────────┐                  
                                #x+3 │  Bare Connector #x  │                    
                                     └─────────────────────┘                       
                          
                        Pool Transaction     

## Lift 🛗
`Lift` is a bare, on-chain transaction output type used for onboarding to the Bitcoin VM. When a `Lift` output is funded and has gained two on-chain confirmations, it can be swapped out for a 1:1 `VTXO` in a process known as lifting. In short, a `Lift` output lifts itself up to a `VTXO`.

`Lift` carries two  spending conditions:
`(Self + Operator) or (Self after 1 month)`

-   Both `Self` and `Operator` must sign from the collaborative path `(Self + Operator)` to forfeit the `Lift` output in exchange for a 1:1 `VTXO`. `Self` swaps out the `Lift` output with the provided `Bare Connector` to receive a new `VTXO` in return.
    
-   In case the `Operator` is non-collaborative and does not sign from the collaborative path, `Self` can trigger the exit path `(Self after 1 month)` to reclaim their funds.

## Bare Connector 🔌
`Bare Connector` is a bare, on-chain transaction output type used for lifting a `Lift` output. `Bare Connector` is a key-path-only `Operator` single-sig. A series of `Bare Connectors` can be included in a `Pool Transaction` and provided to `Self` by the `Operator`.
                                                            
                                                            
                                Prevouts                      Outs          
                         ┌────────────────────┐      ┌────────────────────┐ 
                     #0  │        Lift        │   #0 │      Operator      │
                         └────────────────────┘      └────────────────────┘                    
      From  Pool         ┌────────────────────┐
      Transaction -- #1->│   Bare Connector   │ 
                         └────────────────────┘    
      
                                        Lift Transaction 

## VTXO 💵
`VTXO` is a virtual, off-chain transaction output that holds the `Self` funds. `VTXOs` are projected by the `VTXO Projector` and can be unilaterally redeemed on-chain. A `VTXO` expires three months after its creation, or, in other words, three months after its projector `VTXO Projector` hits on-chain. 

Once a `VTXO` expires, it can no longer be redeemed or claimed on-chain; therefore, `Self` must refresh its `VTXOs` into new ones on a monthly basis. It is the client software's job to abstract the refresh UX away for `Self`. At the protocol level, however, refreshes are interpreted differently from regular transfers, and the `Operator` is not allowed to charge liquidity fees when `VTXOs` are refreshed.

## VTXO Projector 🎥
`VTXO Projector` is a bare, on-chain transaction output type contained in each pool transaction.  `Projector` is used for for projecting `VTXOs` and `Connectors` in a pseudo-covenant manner.
                                                      
                                           ⋰ ┌──────────────────┐
                                         ⋰   │      VTXO #0     │
                                       ⋰     └──────────────────┘
                                     ⋰       ┌──────────────────┐
                                   ⋰         │      VTXO #1     │
        ┌──────────────────┐     ⋰           └──────────────────┘
        │  VTXO Projector  │ 🎥 ⋮                        
        └──────────────────┘     ⋱                     ┊
                                   ⋱                
                                     ⋱       ┌──────────────────┐
                                       ⋱     │      VTXO #n     │
                                         ⋱   └──────────────────┘
                                           ⋱
                                                      
                  

## Payload 📦
`Payload` is a bare, on-chain transaction output type contained in each pool transaction.  `Payload` stores entries, projector signatures, s commitments, and the fresh operator key of the session.

## Connector 🔌
`Connector` is a virtual, off-chain transaction output type projected by the `Projector`.  `Connector` connects `VTXOs` into pool transactions.

## Channel 👥
`Channel` turns `VTXO` into a virtual channel, with a lifetime of 128 state transitions.

## Self 👨‍💻
`Self` is a virtual P2TR output containing the self inner-key with no script-path involved.

## Operator 🏭
`Operator` is a virtual P2TR output containing the operator inner-key with no script-path involved.