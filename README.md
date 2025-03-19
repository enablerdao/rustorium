# Rustorium

Rustorium is a next-generation blockchain platform built in Rust, designed to provide a scalable, user-friendly environment for token creation and management.

## Core Features

- **DAG-based Transaction Processing**: Enables parallel transaction processing for high throughput
- **Avalanche Consensus**: Fast finality with high security
- **Dynamic Sharding**: Automatically scales with network demand
- **Smart Token Generator**: AI-assisted token creation and customization
- **Automated Fee Optimization**: Intelligent fee management across shards
- **Community Governance**: DAO-based decision making for platform parameters

## Quick Start

```bash
# Start development environment with multiple test nodes
cargo run -- --dev --nodes 5

# Start single node in release mode
cargo run --release

# Start API server only
cargo run -- --api-only

# Start frontend only
cargo run -- --frontend-only
```

## Architecture

Rustorium combines three powerful technologies:
1. **DAG (Directed Acyclic Graph)** for parallel transaction processing
2. **Avalanche Consensus** for fast and secure finality
3. **Dynamic Sharding** for unlimited scalability

The platform is designed to handle cross-shard transactions efficiently through our innovative Cross-Shard DAG system.

## Documentation

For detailed documentation, see:
- [Project Structure](docs/project-structure.md)
- [Architecture Overview](docs/architecture.md)
- [Token System](docs/token-system.md)
- [Governance](docs/governance.md)
- [API Reference](docs/api-reference.md)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.