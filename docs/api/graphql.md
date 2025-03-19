# GraphQL API

## Endpoint

```
https://api.rustorium.com/graphql
```

## Authentication

Include your API key in the HTTP headers:

```http
Authorization: Bearer YOUR_API_KEY
```

## Schema

### Types

```graphql
type Token {
    id: ID!
    name: String!
    symbol: String!
    decimals: Int!
    totalSupply: String!
    currentSupply: String!
    price: Float!
    holders: Int!
    transactions: [Transaction!]!
    economics: TokenEconomics!
    metadata: TokenMetadata
}

type TokenEconomics {
    initialPrice: Float!
    inflationRate: Float!
    maxSupply: String!
    fee: Int!
    feeDistribution: FeeDistribution!
    stakingReward: StakingReward!
}

type Transaction {
    id: ID!
    from: String!
    to: String!
    amount: String!
    token: Token!
    fee: String!
    status: TransactionStatus!
    timestamp: DateTime!
    blockNumber: Int
}

type Account {
    address: String!
    balances: [Balance!]!
    transactions: [Transaction!]!
    tokens: [Token!]!
}

type Balance {
    token: Token!
    amount: String!
    valueUsd: Float!
}

enum TransactionStatus {
    PENDING
    CONFIRMED
    REJECTED
    FAILED
}
```

### Queries

```graphql
type Query {
    # Token queries
    token(id: ID!): Token
    tokens(
        first: Int
        after: String
        type: TokenType
        sortBy: TokenSortField
        sortOrder: SortOrder
    ): TokenConnection!
    
    # Transaction queries
    transaction(id: ID!): Transaction
    transactions(
        first: Int
        after: String
        filter: TransactionFilter
    ): TransactionConnection!
    
    # Account queries
    account(address: String!): Account
    accountBalances(address: String!): [Balance!]!
    
    # Market data
    marketStats: MarketStats!
    tokenPrice(id: ID!): TokenPrice!
}
```

### Mutations

```graphql
type Mutation {
    # Token mutations
    createToken(input: CreateTokenInput!): CreateTokenPayload!
    updateToken(input: UpdateTokenInput!): UpdateTokenPayload!
    
    # Transaction mutations
    sendTransaction(input: SendTransactionInput!): SendTransactionPayload!
    cancelTransaction(id: ID!): CancelTransactionPayload!
    
    # Account mutations
    updateAccount(input: UpdateAccountInput!): UpdateAccountPayload!
}
```

### Subscriptions

```graphql
type Subscription {
    # Token subscriptions
    tokenUpdates(id: ID!): TokenUpdate!
    priceUpdates(tokens: [ID!]!): PriceUpdate!
    
    # Transaction subscriptions
    transactionUpdates(filter: TransactionFilter): TransactionUpdate!
    
    # Account subscriptions
    accountUpdates(address: String!): AccountUpdate!
}
```

## Example Queries

### Get Token Information

```graphql
query GetToken($id: ID!) {
    token(id: $id) {
        id
        name
        symbol
        totalSupply
        currentSupply
        price
        holders
        economics {
            initialPrice
            inflationRate
            maxSupply
            fee
        }
        transactions(first: 10) {
            edges {
                node {
                    id
                    from
                    to
                    amount
                    status
                    timestamp
                }
            }
        }
    }
}
```

### Get Account Balances

```graphql
query GetAccountBalances($address: String!) {
    accountBalances(address: $address) {
        token {
            id
            symbol
            price
        }
        amount
        valueUsd
    }
}
```

### Create Token

```graphql
mutation CreateToken($input: CreateTokenInput!) {
    createToken(input: $input) {
        token {
            id
            name
            symbol
            totalSupply
        }
        transaction {
            id
            status
        }
    }
}
```

### Subscribe to Price Updates

```graphql
subscription PriceUpdates($tokens: [ID!]!) {
    priceUpdates(tokens: $tokens) {
        token {
            id
            symbol
        }
        price
        change24h
        volume24h
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
            "message": "Detailed error message",
            "locations": [
                {
                    "line": 2,
                    "column": 3
                }
            ],
            "path": ["token", "transactions"],
            "extensions": {
                "code": "INVALID_REQUEST",
                "details": {
                    "field": "specific_field",
                    "reason": "validation_failed"
                }
            }
        }
    ]
}
```

## Rate Limiting

Rate limits are included in response headers:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Examples

### Node.js Example

```javascript
const { GraphQLClient } = require('graphql-request');

const client = new GraphQLClient('https://api.rustorium.com/graphql', {
    headers: {
        Authorization: 'Bearer YOUR_API_KEY',
    },
});

const query = `
    query GetToken($id: ID!) {
        token(id: $id) {
            id
            name
            symbol
            price
        }
    }
`;

client.request(query, { id: 'token_id' })
    .then(data => console.log(data))
    .catch(error => console.error(error));
```

### Python Example

```python
from gql import gql, Client
from gql.transport.requests import RequestsHTTPTransport

transport = RequestsHTTPTransport(
    url='https://api.rustorium.com/graphql',
    headers={'Authorization': 'Bearer YOUR_API_KEY'}
)

client = Client(transport=transport, fetch_schema_from_transport=True)

query = gql('''
    query GetToken($id: ID!) {
        token(id: $id) {
            id
            name
            symbol
            price
        }
    }
''')

result = client.execute(query, variable_values={'id': 'token_id'})
print(result)
```