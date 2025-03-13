# Nourish Chain Supper Club

A comprehensive platform for food security and community engagement built with Rust and WebAssembly.

## Components

### Social Hub
The social component provides a community interface for engagement, impact tracking, and resource sharing. Built with Leptos and WebAssembly for Next.js integration.

### DEX (Decentralized Exchange)
The DEX component facilitates transparent and efficient exchange of resources, tokens, and community contributions.

### Spirulina Tracking
Monitors spirulina production, harvesting, and distribution to support food security initiatives.

## Architecture

The entire platform is built using Rust with WebAssembly compilation for web integration:

- **Type-Safe**: All components use strong type checking
- **WebAssembly**: Compiled to WASM for performance and security
- **Next.js Integration**: Seamless integration with Next.js frontend
- **Responsive Design**: Works on all device sizes
- **Dark Mode**: Full support for light and dark themes

## Building the Project

Each component can be built separately using the provided build scripts:

\\\
cd social
./build.ps1

cd dex
./build.ps1
\\\

## Integration

See the \
extjs_integration_example.jsx\ files in each component directory for examples of how to integrate these components into a Next.js application.
