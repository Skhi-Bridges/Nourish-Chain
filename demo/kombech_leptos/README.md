# KOMBECH Social Hub

A type-safe Rust implementation of the social media hub for the KOMBECH project using Leptos.

## Features

- **Community Dashboard** - View key metrics for the KOMBECH community
- **Social Media Integration** - Feeds and widgets for Twitter, Discord, Telegram, and YouTube
- **Content Showcase** - Latest updates and news from the project
- **Interactive Elements** - Community Q&A form and impact visualization
- **Dark Mode Support** - Toggle between light and dark themes
- **Responsive Design** - Works on all device sizes

## Type-Safe Implementation

This component is built with a focus on type safety:

- **Type-Safe CSS** - Implemented through the `TailwindClass` and `ResponsiveClass` utilities
- **Type-Safe JavaScript Bindings** - Full TypeScript definitions for the WASM module
- **Type-Safe Next.js Integration** - Rust interfaces for Next.js APIs
- **Type-Safe Props** - All component props are properly typed

## Building and Development

### Prerequisites

- Rust and Cargo
- wasm-pack
- Node.js and npm/yarn (if using with Next.js)

### Building

```bash
# Build the WASM module
wasm-pack build --target web

# For Next.js integration
wasm-pack build --target bundler
```

### Integration with Next.js

The component can be seamlessly integrated with a Next.js application:

```javascript
// pages/social.js
import { useEffect, useState } from 'react';
import init, { mount_social_page } from '@nourish-chain/social-leptos';

export default function SocialPage() {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  
  useEffect(() => {
    async function loadWasm() {
      await init();
      mount_social_page('#social-root', { 
        initialDarkMode: false, 
        language: 'en' 
      });
      setWasmLoaded(true);
    }
    
    loadWasm();
  }, []);
  
  return (
    <div>
      <div id="social-root" />
      {!wasmLoaded && <div>Loading...</div>}
    </div>
  );
}
```

## Project Structure

- `src/lib.rs` - Main component definition and exports
- `src/additional_components.rs` - Social media feed components
- `src/type_safe_components.rs` - Type-safe component utilities
- `src/nextjs_bridge.rs` - Bridge between Leptos and Next.js
- `src/js_types.rs` - Type-safe JavaScript/TypeScript bindings
- `src/next_integration.rs` - Next.js specific integration
- `src/main.rs` - Entry point for standalone usage

## License

Copyright © 2025 KOMBECH DAO. All rights reserved.

