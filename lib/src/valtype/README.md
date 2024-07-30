# Value Types
`Bitcoin Virtual Machine` employs of 8+ value types:

| Value Type        |  Description                                                            |
|:------------------|:------------------------------------------------------------------------|
| Short Val         | Succinct value representation for integers: UInt8-16-24-32.             |
| Long Val          | Succinct value representation for integers: UInt8-16-24-32-40-48-56-64. |
| Account           | Possibly compact account representation.                                |
| Contract          | Possibly compact contract representation.                               |
| Common Account    | Possibly common `Account` representation.                               |
| Common Contract   | Possibly common `Contract` representation.                              |
| Common Short Val  | Possibly common `Short Val` representation.                             |
| Common Long Val   | Possibly common `Long Val` representation.                              |