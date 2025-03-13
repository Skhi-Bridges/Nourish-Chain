import { useEffect, useState } from 'react';
import Head from 'next/head';
import Link from 'next/link';

export default function DEX() {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const [dexModule, setDexModule] = useState(null);
  const [selectedPair, setSelectedPair] = useState({
    token_a: "NRSH",
    token_b: "USDT",
    reserve_a: 10000.0,
    reserve_b: 12800.0,
    price_ratio: 1.28,
    fee: 0.00369, // 0.369%
  });
  
  const availablePairs = [
    {
      token_a: "NRSH",
      token_b: "USDT",
      reserve_a: 10000.0,
      reserve_b: 12800.0,
      price_ratio: 1.28,
      fee: 0.00369,
    },
    {
      token_a: "NRSH",
      token_b: "ETH",
      reserve_a: 25000.0,
      reserve_b: 10.5,
      price_ratio: 0.00042,
      fee: 0.00369,
    },
    {
      token_a: "NRSH",
      token_b: "AVAX",
      reserve_a: 18000.0,
      reserve_b: 600.0,
      price_ratio: 0.0333,
      fee: 0.00369,
    }
  ];

  useEffect(() => {
    async function loadWasm() {
      try {
        // Dynamic import of the wasm module
        const dexLeptos = await import('dex_leptos');
        setDexModule(dexLeptos);
        setWasmLoaded(true);
      } catch (err) {
        console.error('Failed to load WASM module:', err);
      }
    }
    
    loadWasm();
  }, []);

  const renderLeptosComponent = () => {
    if (!wasmLoaded || !dexModule) {
      return <div className="text-center py-8">Loading DEX components...</div>;
    }

    const pairJson = JSON.stringify(selectedPair);
    
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div dangerouslySetInnerHTML={{ 
          __html: dexModule.render_dex_swap_form(pairJson) 
        }} />
        
        <div dangerouslySetInnerHTML={{ 
          __html: dexModule.render_liquidity_pool_card(pairJson) 
        }} />
      </div>
    );
  };

  return (
    <div className="bg-midnight-900 min-h-screen">
      <Head>
        <title>Nourish Chain DEX | Swap and Add Liquidity</title>
        <meta name="description" content="Decentralized exchange for Nourish Chain tokens with 0.369% trading fee" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <header className="bg-midnight-800 border-b border-midnight-700">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-4">
              <h1 className="text-2xl font-bold text-spirulina-500">Nourish Chain</h1>
              <nav className="hidden md:flex gap-6">
                <Link href="/" className="text-gray-300 hover:text-white transition-colors">
                  Dashboard
                </Link>
                <Link href="/dex" className="text-white font-medium border-b-2 border-spirulina-500 pb-1">
                  DEX
                </Link>
                <Link href="/social" className="text-gray-300 hover:text-white transition-colors">
                  Community
                </Link>
                <Link href="/governance" className="text-gray-300 hover:text-white transition-colors">
                  Governance
                </Link>
              </nav>
            </div>
            
            <button className="px-4 py-2 bg-spirulina-600 hover:bg-spirulina-700 rounded-lg text-white font-medium transition-colors">
              Connect Wallet
            </button>
          </div>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold mb-2">Nourish Chain DEX</h1>
          <p className="text-gray-400">Swap and provide liquidity with 0.369% trading fee</p>
        </div>
        
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Select Token Pair</h2>
          <div className="flex flex-wrap gap-4">
            {availablePairs.map((pair, index) => (
              <button 
                key={index}
                className={`px-4 py-2 rounded-lg transition-colors ${
                  selectedPair.token_a === pair.token_a && selectedPair.token_b === pair.token_b
                    ? 'bg-spirulina-600 text-white'
                    : 'bg-midnight-800 text-gray-300 hover:bg-midnight-700'
                }`}
                onClick={() => setSelectedPair(pair)}
              >
                {pair.token_a}/{pair.token_b}
              </button>
            ))}
          </div>
        </div>
        
        {renderLeptosComponent()}
        
        <div className="mt-12 bg-midnight-800 rounded-xl p-6 border border-gold-700/30">
          <h2 className="text-xl font-semibold mb-4 text-spirulina-500">About Nourish Chain DEX</h2>
          <p className="text-gray-300 mb-4">
            The Nourish Chain DEX is a decentralized exchange built on the Nourish Chain blockchain, 
            designed to provide a secure and efficient way to trade NRSH tokens and other assets.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-6">
            <div className="bg-midnight-900/50 p-4 rounded-lg">
              <h3 className="font-medium text-gold-500 mb-2">0.369% Trading Fee</h3>
              <p className="text-gray-400 text-sm">
                Our low 0.369% trading fee ensures cost-effective trading while supporting the Nourish Chain ecosystem.
              </p>
            </div>
            <div className="bg-midnight-900/50 p-4 rounded-lg">
              <h3 className="font-medium text-gold-500 mb-2">Earn By Providing Liquidity</h3>
              <p className="text-gray-400 text-sm">
                Earn a portion of trading fees by providing liquidity to our pools and receive LP tokens.
              </p>
            </div>
            <div className="bg-midnight-900/50 p-4 rounded-lg">
              <h3 className="font-medium text-gold-500 mb-2">Integrated with Eigenlayer</h3>
              <p className="text-gray-400 text-sm">
                Our DEX leverages Eigenlayer for enhanced security and capital efficiency.
              </p>
            </div>
          </div>
        </div>
      </main>

      <footer className="bg-midnight-800 border-t border-midnight-700 mt-12">
        <div className="container mx-auto px-4 py-8">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div>
              <h3 className="text-xl font-bold text-spirulina-500 mb-4">Nourish Chain</h3>
              <p className="text-gray-400">
                Making superfoods accessible and affordable through blockchain technology.
              </p>
            </div>
            <div>
              <h4 className="font-medium text-white mb-4">Quick Links</h4>
              <ul className="space-y-2">
                <li><Link href="/" className="text-gray-400 hover:text-white transition-colors">Home</Link></li>
                <li><Link href="/dex" className="text-gray-400 hover:text-white transition-colors">DEX</Link></li>
                <li><Link href="/governance" className="text-gray-400 hover:text-white transition-colors">Governance</Link></li>
                <li><Link href="/docs" className="text-gray-400 hover:text-white transition-colors">Documentation</Link></li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium text-white mb-4">Community</h4>
              <ul className="space-y-2">
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Discord</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Twitter</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Telegram</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">GitHub</a></li>
              </ul>
            </div>
          </div>
          <div className="mt-8 pt-8 border-t border-midnight-700 text-center text-gray-500">
            <p>© {new Date().getFullYear()} Nourish Chain DAO. All rights reserved.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}
