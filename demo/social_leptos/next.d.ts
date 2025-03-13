// Type definitions for the Rust WebAssembly bindings

// Social Leptos module
declare module "social-leptos" {
  // Main rendering functions
  export function render_social_page(): string;
  export function mount_social_page(selector: string): void;
  
  // Type-safe Next.js integration
  export interface SocialPageProps {
    initialDarkMode?: boolean;
    language?: string;
  }
  
  export class NextIntegration {
    constructor(options: SocialPageProps);
    render(): string;
  }
  
  // Navigation helpers
  export function navigate_to(path: string): void;
  
  // Event handlers
  export function register_hydration_listener(): void;
  
  // For React integration
  export interface ReactProps {
    initialDarkMode: boolean;
    language: string;
  }
  
  export interface ReactEvent {
    preventDefault(): void;
    target: any;
  }
  
  // Create a custom event handler
  export function create_event_handler(handler: (event: ReactEvent) => void): Function;
}

// React component types for Next.js
declare namespace React {
  interface FunctionComponent<P = {}> {
    (props: P): React.ReactElement<P> | null;
  }
  
  type FC<P = {}> = FunctionComponent<P>;
}

// Next.js component with Rust WASM
declare module "*.wasm" {
  const content: any;
  export default content;
}

// Make TypeScript aware of our Rust exports
declare global {
  interface Window {
    wasm_bindgen: (module: any) => Promise<any>;
    social_leptos: typeof import("social-leptos");
  }
}
