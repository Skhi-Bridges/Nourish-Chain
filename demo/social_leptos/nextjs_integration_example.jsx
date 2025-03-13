// Example Next.js component that integrates with the type-safe Rust implementation
// Place this in your Next.js pages directory (e.g., pages/social.jsx)

import { useEffect, useState, useRef } from 'react';
import Head from 'next/head';

// In a real implementation, this would be imported from your built WASM package
// import init, { mount_social_page, render_social_page } from '@nourish-chain/social-leptos';

export default function SocialPage() {
  const [isLoading, setIsLoading] = useState(true);
  const [isClient, setIsClient] = useState(false);
  const socialRootRef = useRef(null);
  const [initialDarkMode, setInitialDarkMode] = useState(false);

  // Check if we're running on the client
  useEffect(() => {
    setIsClient(true);
  }, []);

  // Load and initialize the WASM module
  useEffect(() => {
    if (!isClient) return;

    async function loadWasmModule() {
      try {
        // In a real implementation, this would load the actual WASM module
        // await init();
        console.log('WASM module loaded');
        
        // Get user's preferred color scheme
        const prefersDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
        setInitialDarkMode(prefersDarkMode);
        
        // Mount the component with type-safe props
        // mount_social_page('#social-root', { 
        //   initialDarkMode: prefersDarkMode, 
        //   language: navigator.language || 'en'
        // });
        
        // In this example, we're just simulating the mount
        console.log('Social component mounted with props:', {
          initialDarkMode: prefersDarkMode,
          language: navigator.language || 'en'
        });
        
        setIsLoading(false);
      } catch (error) {
        console.error('Failed to load WASM module:', error);
      }
    }

    loadWasmModule();
  }, [isClient]);

  // Get server-side rendered HTML
  function getServerRenderedHtml() {
    // In a real implementation, this would call the actual Rust function
    // return render_social_page({ 
    //   initialDarkMode, 
    //   language: 'en' 
    // });
    
    // For this example, we return placeholder HTML
    return `
      <div class="min-h-screen p-8 transition-colors duration-300 bg-white text-gray-900">
        <div class="max-w-6xl mx-auto">
          <h2 class="text-2xl font-bold">Community Dashboard</h2>
          <p>This is a server-rendered placeholder for the Rust/WASM component.</p>
        </div>
      </div>
    `;
  }

  return (
    <>
      <Head>
        <title>Nourish Chain - Community Hub</title>
        <meta name="description" content="Join the Nourish Chain community" />
      </Head>

      {/* Loading state */}
      {isLoading && (
        <div className="flex justify-center items-center min-h-screen">
          <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-green-500"></div>
        </div>
      )}

      {/* Mount point for the Rust component */}
      <div 
        id="social-root" 
        ref={socialRootRef}
        dangerouslySetInnerHTML={isClient ? undefined : { __html: getServerRenderedHtml() }}
        style={{ display: isLoading ? 'none' : 'block' }}
      />
    </>
  );
}
