# GraphQL API Reference

Rustorium provides a GraphQL API for flexible and efficient data querying. This document describes the available types, queries, mutations, and subscriptions.

## Endpoint

The GraphQL endpoint is available at:
```
http://localhost:9071/graphql
```

GraphiQL IDE is available at:
```
http://localhost:9071/graphiql
```

## Authentication

Authentication is performed using the `Authorization` header:

```http
Authorization: Bearer <API_KEY>
```

## Schema

### Types

```graphql
type Block {
  number: Int!
  hash: String!
  parentHash: String!
  timestamp: DateTime!
  transactions: [Transaction!]!
  stateRoot: String!
  receiptsRoot: String!
  size: Int!
  gasUsed: Int!
  gasLimit: Int!
}

type Transaction {
  hash: String!
  blockNumber: Int
  from: String!
  to: String!
  value: BigInt!
  data: String!
  nonce: Int!
  status: TxStatus!
  timestamp: DateTime!
  gasUsed: Int!
  gasPrice: BigInt!
  confirmations: Int!
}

type State {
  key: String!
  value: String!
  proof: MerkleProof!
  blockNumber: Int!
  timestamp: DateTime!
}

type MerkleProof {
  root: String!
  proof: [String!]!
}

enum TxStatus {
  PENDING
  CONFIRMED
  FAILED
}

scalar DateTime
scalar BigInt
```

### Queries

```graphql
type Query {
  # Block queries
  block(number: Int!): Block
  blocks(
    start: Int!
    end: Int!
    orderBy: BlockOrderBy = NUMBER_DESC
  ): [Block!]!
  latestBlock: Block!
  
  # Transaction queries
  transaction(hash: String!): Transaction
  transactions(
    address: String
    status: TxStatus
    limit: Int = 10
    offset: Int = 0
    orderBy: TxOrderBy = TIMESTAMP_DESC
  ): TransactionConnection!
  
  # State queries
  state(key: String!): State
  states(
    keys: [String!]!
    blockNumber: Int
  ): [State!]!
  
  # Node queries
  nodeInfo: NodeInfo!
  metrics: Metrics!
}

type TransactionConnection {
  edges: [TransactionEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type TransactionEdge {
  node: Transaction!
  cursor: String!
}

type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}

enum BlockOrderBy {
  NUMBER_ASC
  NUMBER_DESC
  TIMESTAMP_ASC
  TIMESTAMP_DESC
}

enum TxOrderBy {
  TIMESTAMP_ASC
  TIMESTAMP_DESC
  VALUE_ASC
  VALUE_DESC
}
```

### Mutations

```graphql
type Mutation {
  # Transaction mutations
  submitTransaction(input: TxInput!): TxReceipt!
  
  # State mutations
  updateState(input: StateInput!): StateUpdateReceipt!
  
  # Node mutations
  updateNodeConfig(input: NodeConfigInput!): NodeConfig!
}

input TxInput {
  to: String!
  value: BigInt!
  data: String
  nonce: Int
  gasLimit: Int
  gasPrice: BigInt
}

input StateInput {
  key: String!
  value: String!
  proof: MerkleProofInput
}

input MerkleProofInput {
  root: String!
  proof: [String!]!
}

input NodeConfigInput {
  maxPeers: Int
  maxPendingTx: Int
  blockTime: Int
}
```

### Subscriptions

```graphql
type Subscription {
  # Block subscriptions
  onNewBlock: Block!
  onChainReorg(depth: Int = 1): ChainReorg!
  
  # Transaction subscriptions
  onTxConfirmed(hash: String): Transaction!
  onTxFailed(hash: String): Transaction!
  
  # State subscriptions
  onStateUpdate(key: String): State!
}

type ChainReorg {
  oldBlocks: [Block!]!
  newBlocks: [Block!]!
  commonAncestor: Block!
}
```

## Examples

### Query Examples

Get latest block:
```graphql
query {
  latestBlock {
    number
    hash
    timestamp
    transactions {
      hash
      status
    }
  }
}
```

Get transaction details:
```graphql
query GetTransaction($hash: String!) {
  transaction(hash: $hash) {
    hash
    blockNumber
    from
    to
    value
    status
    confirmations
    timestamp
  }
}
```

Get multiple states:
```graphql
query GetStates($keys: [String!]!) {
  states(keys: $keys) {
    key
    value
    blockNumber
    proof {
      root
      proof
    }
  }
}
```

### Mutation Examples

Submit transaction:
```graphql
mutation SubmitTx($input: TxInput!) {
  submitTransaction(input: $input) {
    hash
    status
    timestamp
  }
}
```

Update state:
```graphql
mutation UpdateState($input: StateInput!) {
  updateState(input: $input) {
    key
    value
    blockNumber
  }
}
```

### Subscription Examples

Subscribe to new blocks:
```graphql
subscription {
  onNewBlock {
    number
    hash
    transactions {
      hash
      status
    }
  }
}
```

Subscribe to transaction confirmations:
```graphql
subscription OnTxConfirmed($hash: String!) {
  onTxConfirmed(hash: $hash) {
    hash
    blockNumber
    confirmations
    timestamp
  }
}
```

## Error Handling

GraphQL errors follow this format:

```json
{
  "errors": [
    {
      "message": "Error description",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": ["transaction", "hash"],
      "extensions": {
        "code": "INVALID_HASH",
        "details": "Additional error details"
      }
    }
  ]
}
```

Common error codes:

| Code | Description | Solution |
|------|-------------|----------|
| INVALID_INPUT | Invalid input data | Check input parameters |
| NOT_FOUND | Resource not found | Verify resource exists |
| UNAUTHORIZED | Authentication failed | Check API key |
| FORBIDDEN | Permission denied | Check permissions |
| RATE_LIMITED | Too many requests | Reduce request rate |

## Rate Limiting

- 500 queries per minute
- 100 mutations per minute
- 50 subscriptions per connection
- 1000 subscription events per minute

## Best Practices

1. **Query Optimization**
   - Request only needed fields
   - Use pagination for large results
   - Batch related queries

2. **Caching**
   - Use persisted queries
   - Implement client-side caching
   - Consider field-level caching

3. **Error Handling**
   - Handle partial results
   - Implement retry logic
   - Log errors for debugging

4. **Security**
   - Validate input data
   - Set query complexity limits
   - Use HTTPS in production

## Tools and SDKs

### Official SDKs

- [Rust](https://github.com/enablerdao/rustorium-rs)
- [TypeScript](https://github.com/enablerdao/rustorium-ts)
- [Python](https://github.com/enablerdao/rustorium-py)
- [Go](https://github.com/enablerdao/rustorium-go)

### Development Tools

- [GraphiQL](http://localhost:9071/graphiql)
- [Apollo Studio](https://studio.apollographql.com)
- [GraphQL Playground](https://github.com/graphql/graphql-playground)

## Support

If you need help with the GraphQL API:

1. Check the [FAQ](../appendix/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
