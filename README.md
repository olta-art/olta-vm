## About
`olta-vm` is a workspace that contains the building blocks of Olta's engine, responsible for document instructions computation, lobbies management, optimistic persistant storage, ao compute & syncing, and VM <> client communication over WebSockets.


## olta-vm & HyperBEAM

This workpace is being designed to be shipped as a HyperBEAM device in its final stage. olta-vm is the intersection between HyperBEAM's ao compute, real-time interactive arts, bleeding-edge performance, and Arweave's tamper-proof immutable provenance.

## Roadmap

| crate  | description | status|
| :-------------: |:-------------:| :-------------:|
| [`vm`](./crates/vm/)      | the Lobby's instructions calculator     | `v0.1.0` |
| [`storage`](./crates/storage/)      | persistant storage for optimistic compute results     | `v0.1.0` |
| [`server`](./crates/server/)      | the vm's API - over websockers     | `v0.1.0` |
| `ao`      | the compute truth source & provenance     | `wip` |

## Benchmarks

* server endpoint `wss://olta-vm.load.network/`

### CrateDocument

```bash
Paylod: CreateDocument
Ping:    335.59 µs | Avg:    520.90 µs | Jitter:    236.20 µs | FPS:  2979.8     

--- Final Stats ---
Samples: 3
Average: 520.90 µs
Min: 335.59 µs
Max: 854.24 µs
Jitter (stdev): 236.20 µs
```

## License 

This project is licensed under the [Apache License, Version 2.0](./LICENSE-APACHE) License