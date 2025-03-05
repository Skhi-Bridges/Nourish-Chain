import { useEffect, useState } from 'react';
import Head from 'next/head';
import Link from 'next/link';

export default function SocialPage() {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const [socialModule, setSocialModule] = useState(null);
  const [socialHtml, setSocialHtml] = useState('');

  // Load WASM module
  useEffect(() => {
    async function loadWasm() {
      try {
        // In a real implementation, this would be properly packaged and built
        // For demo purposes, we're simulating the WASM loading
        const socialModuleImport = {
          render_social_page: () => {
            // This is a placeholder - the actual WASM module would render the component
            return '<div id="social-page-container"></div>';
          }
        };
        
        setSocialModule(socialModuleImport);
        setWasmLoaded(true);
        setSocialHtml(socialModuleImport.render_social_page());
      } catch (err) {
        console.error("Failed to load WASM module:", err);
      }
    }

    loadWasm();
  }, []);

  return (
    <div className="min-h-screen bg-midnight-900 text-white">
      <Head>
        <title>Nourish Chain - Community Hub</title>
        <meta name="description" content="Join the Nourish Chain community and stay connected with updates, events, and fellow members" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      {/* Navigation */}
      <nav className="bg-midnight-800 border-b border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <span className="text-2xl font-bold text-spirulina-500">NRSH</span>
              </div>
              <div className="hidden md:block">
                <div className="ml-10 flex items-baseline space-x-4">
                  <Link href="/" className="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                    Dashboard
                  </Link>
                  <Link href="/staking" className="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                    Staking
                  </Link>
                  <Link href="/dex" className="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                    DEX
                  </Link>
                  <Link href="/social" className="bg-spirulina-600 text-white px-3 py-2 rounded-md text-sm font-medium">
                    Community
                  </Link>
                </div>
              </div>
            </div>
            <div>
              <button className="bg-spirulina-500 hover:bg-spirulina-600 text-white px-4 py-2 rounded-md text-sm font-medium transition">
                Connect Wallet
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* Main content */}
      <main>
        {/* Loading state */}
        {!wasmLoaded && (
          <div className="flex justify-center items-center h-96">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-spirulina-500"></div>
          </div>
        )}

        {/* Fallback UI while WASM loads/develops */}
        {wasmLoaded && (
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            {/* Community Dashboard */}
            <div className="bg-spirulina-500 p-6 rounded-xl shadow-lg">
              <h2 className="text-2xl font-bold mb-4">Community Dashboard</h2>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="bg-white bg-opacity-10 p-4 rounded-lg">
                  <h3 className="text-sm font-semibold text-opacity-80">Members</h3>
                  <p className="text-2xl font-bold mt-2">2,450</p>
                </div>
                <div className="bg-white bg-opacity-10 p-4 rounded-lg">
                  <h3 className="text-sm font-semibold text-opacity-80">Active Validators</h3>
                  <p className="text-2xl font-bold mt-2">142</p>
                </div>
                <div className="bg-white bg-opacity-10 p-4 rounded-lg">
                  <h3 className="text-sm font-semibold text-opacity-80">Staked NRSH</h3>
                  <p className="text-2xl font-bold mt-2">12.45M</p>
                </div>
                <div className="bg-white bg-opacity-10 p-4 rounded-lg">
                  <h3 className="text-sm font-semibold text-opacity-80">Total Nodes</h3>
                  <p className="text-2xl font-bold mt-2">89</p>
                </div>
              </div>
            </div>

            {/* Social Media Grid */}
            <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6 mt-12">
              {/* Twitter Feed */}
              <div className="bg-[#1DA1F2] p-4 rounded-xl h-96 overflow-y-auto">
                <h3 className="text-lg font-bold mb-4">Latest Tweets</h3>
                <div className="space-y-4">
                  <div className="border-b border-white border-opacity-10 pb-3 mb-3">
                    <div className="flex items-center">
                      <div className="w-10 h-10 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                        <span className="text-lg">🌱</span>
                      </div>
                      <div className="ml-2">
                        <p className="font-bold text-sm">@NRSH_ELXR</p>
                        <p className="text-xs text-gray-200">2h ago</p>
                      </div>
                    </div>
                    <p className="mt-2 text-sm">Excited to announce our new partnership with EigenLayer for enhanced validator security! #blockchain #sustainability</p>
                  </div>
                  <div className="border-b border-white border-opacity-10 pb-3 mb-3">
                    <div className="flex items-center">
                      <div className="w-10 h-10 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                        <span className="text-lg">🌱</span>
                      </div>
                      <div className="ml-2">
                        <p className="font-bold text-sm">@NRSH_ELXR</p>
                        <p className="text-xs text-gray-200">6h ago</p>
                      </div>
                    </div>
                    <p className="mt-2 text-sm">Join our next community call to discuss the latest spirulina cultivation techniques. Link in bio!</p>
                  </div>
                </div>
              </div>

              {/* Discord Widget */}
              <div className="bg-[#5865F2] p-4 rounded-xl h-96 overflow-y-auto">
                <h3 className="text-lg font-bold mb-4">Discord Community</h3>
                <div className="bg-white bg-opacity-10 rounded-lg p-4 mb-4">
                  <div className="flex justify-between items-center">
                    <div>
                      <h4 className="font-bold">Nourish Chain DAO</h4>
                      <p className="text-sm">3,245 members online</p>
                    </div>
                    <div className="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                      <span className="text-xl">🌿</span>
                    </div>
                  </div>
                  <a href="#" 
                     className="mt-4 block w-full bg-white bg-opacity-20 hover:bg-opacity-30 text-center py-2 rounded-md font-medium transition">
                    Join Server
                  </a>
                </div>
              </div>

              {/* Telegram Widget */}
              <div className="bg-[#0088cc] p-4 rounded-xl h-96 overflow-y-auto">
                <h3 className="text-lg font-bold mb-4">Telegram Group</h3>
                <div className="bg-white bg-opacity-10 rounded-lg p-4">
                  <div className="flex justify-between items-center">
                    <div>
                      <h4 className="font-bold">NRSH Community</h4>
                      <p className="text-sm">1,876 members</p>
                    </div>
                    <div className="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                      <span className="text-xl">📱</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* YouTube Feed */}
              <div className="bg-[#FF0000] p-4 rounded-xl h-96 overflow-y-auto">
                <h3 className="text-lg font-bold mb-4">YouTube Channel</h3>
                <div className="space-y-4">
                  <div className="bg-white bg-opacity-10 rounded-lg overflow-hidden">
                    <div className="h-32 bg-black flex items-center justify-center">
                      <div className="w-12 h-12 rounded-full bg-white bg-opacity-20 flex items-center justify-center">
                        <span className="text-xl">▶️</span>
                      </div>
                    </div>
                    <div className="p-3">
                      <h4 className="font-medium text-sm">Nourish Chain: Revolutionizing Food Security</h4>
                      <p className="text-xs text-gray-300 mt-1">32K views • 2 weeks ago</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Content Showcase */}
            <div className="mt-12">
              <h2 className="text-2xl font-bold mb-6">Latest Updates</h2>
              <div className="grid md:grid-cols-3 gap-6">
                <div className="bg-midnight-800 rounded-xl overflow-hidden shadow-lg">
                  <div className="h-48 bg-spirulina-500 flex items-center justify-center">
                    <span className="text-4xl">📈</span>
                  </div>
                  <div className="p-6">
                    <h3 className="font-bold text-lg mb-2">Q1 Growth Report</h3>
                    <p className="text-gray-400 text-sm mb-4">
                      The NRSH ecosystem grew by 38% this quarter with significant expansion in Southeast Asia.
                    </p>
                    <a href="#" className="text-spirulina-500 text-sm font-medium hover:underline">Read Full Report →</a>
                  </div>
                </div>
              </div>
            </div>

            {/* Join the Movement */}
            <div className="mt-16 mb-8">
              <h2 className="text-2xl font-bold mb-6">Join the Movement</h2>
              <div className="grid md:grid-cols-2 gap-8">
                <div className="bg-midnight-800 p-6 rounded-xl">
                  <h3 className="text-xl font-bold mb-4">Ask the Community</h3>
                  <form>
                    <div className="mb-4">
                      <label htmlFor="email" className="block text-sm font-medium mb-1">Your Email</label>
                      <input
                        type="email"
                        id="email"
                        className="w-full p-2 rounded-md bg-midnight-900 border border-gray-700 focus:border-spirulina-500 focus:outline-none transition"
                        required
                      />
                    </div>
                    <div className="mb-4">
                      <label htmlFor="question" className="block text-sm font-medium mb-1">Your Question</label>
                      <textarea
                        id="question"
                        className="w-full p-2 rounded-md bg-midnight-900 border border-gray-700 focus:border-spirulina-500 focus:outline-none transition h-24"
                        required
                      ></textarea>
                    </div>
                    <button
                      type="submit"
                      className="w-full bg-spirulina-500 hover:bg-spirulina-600 text-white py-2 px-4 rounded-md font-medium transition"
                    >
                      Submit Question
                    </button>
                  </form>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* WASM container - in a real app, this would be populated by the WASM output */}
        <div id="social-page-container" dangerouslySetInnerHTML={{ __html: socialHtml }} />
      </main>

      {/* Footer */}
      <footer className="bg-midnight-800 border-t border-gray-700 py-8">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <h3 className="text-xl font-bold text-spirulina-500 mb-4">Nourish Chain</h3>
              <p className="text-gray-400 text-sm">Revolutionizing food security through blockchain technology and sustainable spirulina cultivation.</p>
            </div>
            <div>
              <h4 className="font-bold mb-4">Resources</h4>
              <ul className="space-y-2">
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Documentation</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Whitepaper</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">GitHub</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Community</h4>
              <ul className="space-y-2">
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Discord</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Twitter</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Telegram</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Legal</h4>
              <ul className="space-y-2">
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Privacy Policy</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Terms of Service</a></li>
              </ul>
            </div>
          </div>
          <div className="mt-8 pt-8 border-t border-gray-700 text-center text-gray-400 text-sm">
            &copy; {new Date().getFullYear()} Nourish Chain DAO. All rights reserved.
          </div>
        </div>
      </footer>
    </div>
  );
}
