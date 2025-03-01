AntiPattern: Client->server streaming
https://stackoverflow.com/questions/56766921/multiple-unary-rpc-calls-vs-long-running-bidirectional-streaming-in-grpc
Be very critical in considering if you need a streaming RPC for your use case. gRPC already uses HTTP/2 streams under the hood, so you already get the performance of persistent and multiplexed connections for unary RPCs.
As an example from industry, Uber (who uses gRPC heavily) has "strongly discouraged" using streaming RPCs stating that it is rarely worth the complexity.
https://github.com/uber/prototool/blob/dev/style/README.md#streaming-rpcs
https://github.com/twitchtv/twirp/issues/70#issuecomment-470367807
Example RPC(abstract): Subscribe to periodic updates
Example RPC: discover traffic info
Example RPC: CRUD updates
