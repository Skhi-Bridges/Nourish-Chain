import React, { useState } from 'react';
import Head from 'next/head';
import Link from 'next/link';
import { Chart as ChartJS, ArcElement, Tooltip, Legend, CategoryScale, LinearScale, PointElement, LineElement, Title } from 'chart.js';
import { Doughnut } from 'react-chartjs-2';

// Register ChartJS components
ChartJS.register(ArcElement, Tooltip, Legend, CategoryScale, LinearScale, PointElement, LineElement, Title);

// Mock data
const mockStakingData = {
  totalStaked: 47500,
  yourStake: 1250,
  eigenlayerValidators: 24,
  validators: [
    { id: 1, name: 'Atlas Validator', stake: 12500, apy: 6.2, isEigenlayer: true },
    { id: 2, name: 'Polaris Node', stake: 8750, apy: 5.9, isEigenlayer: true },
    { id: 3, name: 'Quantum Stake', stake: 6800, apy: 5.7, isEigenlayer: false },
    { id: 4, name: 'Nourish Primary', stake: 19450, apy: 6.5, isEigenlayer: true },
    { id: 5, name: 'EcoStake Validator', stake: 5200, apy: 5.5, isEigenlayer: false },
  ],
  yourNFTs: [
    { id: 1, name: 'Spirulina Grower #042', image: 'https://via.placeholder.com/150/22c55e/ffffff?text=NFT+042', rarity: 'Rare', traits: ['Efficient', 'Nutrient-Rich'] },
    { id: 2, name: 'Harvest Master #107', image: 'https://via.placeholder.com/150/f59e0b/ffffff?text=NFT+107', rarity: 'Epic', traits: ['Fast-Growing', 'High-Yield', 'Resilient'] },
    { id: 3, name: 'Kombucha Creator #033', image: 'https://via.placeholder.com/150/9333ea/ffffff?text=NFT+033', rarity: 'Uncommon', traits: ['Flavorful', 'Quick-Brewing'] },
  ],
  availableRewards: 78.34,
  stakeApportionments: [
    { name: 'Spirulina Farms', percentage: 65 },
    { name: 'Kombucha Production', percentage: 20 },
    { name: 'Protocol Development', percentage: 10 },
    { name: 'Charity Fund', percentage: 5 },
  ],
};

export default function Staking() {
  const [stakingData] = useState(mockStakingData);
  const [stakingAmount, setStakingAmount] = useState('');
  const [selectedValidator, setSelectedValidator] = useState(stakingData.validators[0].id);

  // Chart data for stake apportionments
  const apportionmentData = {
    labels: stakingData.stakeApportionments.map(item => item.name),
    datasets: [
      {
        data: stakingData.stakeApportionments.map(item => item.percentage),
        backgroundColor: ['#22c55e', '#9333ea', '#3b82f6', '#f59e0b'],
        borderColor: ['#16a34a', '#7e22ce', '#2563eb', '#d97706'],
        borderWidth: 1,
      },
    ],
  };

  const handleStake = (e) => {
    e.preventDefault();
    alert(`Staked ${stakingAmount} NRSH with validator ID: ${selectedValidator}`);
    setStakingAmount('');
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-midnight-900 via-midnight-800 to-midnight-900">
      <Head>
        <title>Nourish Chain - Eigenlayer Staking</title>
        <meta name="description" content="Nourish Chain DAO Eigenlayer Staking Dashboard" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <header className="bg-midnight-800 border-b border-gold-600">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center">
              <span className="text-spirulina-500 text-3xl font-bold">NOURISH CHAIN</span>
              <span className="ml-2 text-gold-500 text-sm">DAO</span>
            </div>
            <nav className="hidden md:flex space-x-8">
              <Link href="/" className="text-gray-300 font-medium hover:text-spirulina-400">
                Spirulina Metrics
              </Link>
              <Link href="/staking" className="text-white font-medium hover:text-spirulina-400 border-b-2 border-spirulina-500">
                Eigenlayer Staking
              </Link>
              <Link href="/dex" className="text-gray-300 font-medium hover:text-spirulina-400">
                DEX (0.369%)
              </Link>
            </nav>
            <button className="bg-spirulina-600 text-white px-4 py-2 rounded-lg hover:bg-spirulina-700 transition-colors">
              Connect Wallet
            </button>
          </div>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left column - Staking Interface */}
          <div className="lg:col-span-2 space-y-6">
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-2xl font-bold text-spirulina-500 mb-4">Eigenlayer Staking</h2>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* Staking Overview */}
                <div className="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                  <h3 className="text-lg font-semibold text-spirulina-400 mb-3">Network Stats</h3>
                  
                  <div className="space-y-3">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Total Staked</span>
                      <span className="text-white font-medium">{stakingData.totalStaked.toLocaleString()} NRSH</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Your Stake</span>
                      <span className="text-white font-medium">{stakingData.yourStake.toLocaleString()} NRSH</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Eigenlayer Validators</span>
                      <span className="text-white font-medium">{stakingData.eigenlayerValidators}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Available Rewards</span>
                      <span className="text-spirulina-500 font-medium">{stakingData.availableRewards} NRSH</span>
                    </div>
                    
                    <button className="mt-2 w-full py-2 bg-gold-600 hover:bg-gold-700 text-white font-medium rounded transition-colors">
                      Claim Rewards
                    </button>
                  </div>
                </div>
                
                {/* Stake Apportionment Chart */}
                <div className="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                  <h3 className="text-lg font-semibold text-spirulina-400 mb-3">Stake Allocation</h3>
                  
                  <div className="flex justify-center">
                    <div className="w-48 h-48">
                      <Doughnut 
                        data={apportionmentData} 
                        options={{
                          plugins: {
                            legend: {
                              position: 'bottom',
                              labels: {
                                color: 'rgba(255, 255, 255, 0.7)',
                                font: {
                                  size: 11
                                },
                                padding: 10
                              }
                            },
                            tooltip: {
                              callbacks: {
                                label: function(context) {
                                  return `${context.label}: ${context.raw}%`;
                                }
                              }
                            }
                          }
                        }} 
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            {/* Validator Selection */}
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Validator Selection</h2>
              
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-midnight-700">
                  <thead>
                    <tr>
                      <th className="px-3 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Validator</th>
                      <th className="px-3 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider">Total Stake</th>
                      <th className="px-3 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider">APY</th>
                      <th className="px-3 py-3 text-center text-xs font-medium text-gray-400 uppercase tracking-wider">Type</th>
                      <th className="px-3 py-3 text-center text-xs font-medium text-gray-400 uppercase tracking-wider">Select</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-midnight-700">
                    {stakingData.validators.map((validator) => (
                      <tr key={validator.id} className={selectedValidator === validator.id ? "bg-midnight-700/50" : ""}>
                        <td className="px-3 py-3 whitespace-nowrap">
                          <div className="text-sm font-medium text-white">{validator.name}</div>
                        </td>
                        <td className="px-3 py-3 whitespace-nowrap text-right">
                          <div className="text-sm text-white">{validator.stake.toLocaleString()} NRSH</div>
                        </td>
                        <td className="px-3 py-3 whitespace-nowrap text-right">
                          <div className="text-sm text-spirulina-500 font-medium">{validator.apy}%</div>
                        </td>
                        <td className="px-3 py-3 whitespace-nowrap text-center">
                          {validator.isEigenlayer ? (
                            <span className="px-2 py-1 text-xs font-medium rounded-full bg-gold-700/30 text-gold-300">
                              Eigenlayer
                            </span>
                          ) : (
                            <span className="px-2 py-1 text-xs font-medium rounded-full bg-spirulina-700/30 text-spirulina-300">
                              Standard
                            </span>
                          )}
                        </td>
                        <td className="px-3 py-3 whitespace-nowrap text-center">
                          <input 
                            type="radio" 
                            name="validator" 
                            checked={selectedValidator === validator.id}
                            onChange={() => setSelectedValidator(validator.id)}
                            className="w-4 h-4 text-spirulina-600 border-midnight-600 focus:ring-spirulina-500"
                          />
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              
              {/* Staking Form */}
              <form onSubmit={handleStake} className="mt-6 flex flex-col md:flex-row gap-4">
                <div className="flex-1">
                  <label htmlFor="stakeAmount" className="block text-sm font-medium text-gray-400 mb-1">
                    Amount to Stake (NRSH)
                  </label>
                  <input
                    type="number"
                    id="stakeAmount"
                    value={stakingAmount}
                    onChange={(e) => setStakingAmount(e.target.value)}
                    min="1"
                    required
                    className="w-full py-2 px-3 bg-midnight-700 border border-midnight-600 rounded-lg focus:ring-spirulina-500 focus:border-spirulina-500 text-white"
                    placeholder="Enter amount"
                  />
                </div>
                <div className="flex-none self-end">
                  <button 
                    type="submit" 
                    className="w-full md:w-auto px-6 py-2 bg-spirulina-600 hover:bg-spirulina-700 text-white font-medium rounded-lg transition-colors"
                  >
                    Stake NRSH
                  </button>
                </div>
              </form>
            </div>
          </div>
          
          {/* Right column - NFT Gallery */}
          <div className="space-y-6">
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Your NFT Collection</h2>
              
              <div className="space-y-4">
                {stakingData.yourNFTs.map((nft) => (
                  <div key={nft.id} className="bg-midnight-900/50 rounded-lg border border-midnight-700 overflow-hidden">
                    <div className="aspect-square">
                      <img src={nft.image} alt={nft.name} className="w-full h-full object-cover" />
                    </div>
                    <div className="p-3">
                      <h3 className="text-white font-medium">{nft.name}</h3>
                      <div className="flex justify-between items-center mt-1">
                        <span className={`text-xs px-2 py-0.5 rounded-full ${
                          nft.rarity === 'Epic' 
                            ? 'bg-purple-900/40 text-purple-300'
                            : nft.rarity === 'Rare'
                              ? 'bg-blue-900/40 text-blue-300'
                              : 'bg-green-900/40 text-green-300'
                        }`}>
                          {nft.rarity}
                        </span>
                        <span className="text-xs text-gray-400">#ID: {nft.id}</span>
                      </div>
                      <div className="mt-2 flex flex-wrap gap-1">
                        {nft.traits.map((trait, idx) => (
                          <span key={idx} className="text-xs px-2 py-0.5 bg-midnight-700 text-gray-300 rounded">
                            {trait}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                ))}
                
                <div className="mt-4 pt-4 border-t border-midnight-700">
                  <button className="block w-full text-center bg-gold-600 hover:bg-gold-700 text-white font-medium py-2 rounded-lg transition-colors">
                    Mint New NFT
                  </button>
                </div>
              </div>
            </div>
            
            {/* Eigenlayer Info */}
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Eigenlayer Integration</h2>
              
              <div className="text-sm text-gray-300 space-y-3">
                <p>
                  Nourish Chain uses Eigenlayer for secure, decentralized validator restaking, 
                  enhancing network security while maximizing your staking rewards.
                </p>
                <p>
                  When you stake with Eigenlayer validators, your assets help secure multiple networks 
                  simultaneously while earning additional rewards.
                </p>
                
                <div className="mt-4 pt-4 border-t border-midnight-700">
                  <a 
                    href="https://www.eigenlayer.xyz/" 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="block w-full text-center bg-midnight-700 hover:bg-midnight-600 text-white font-medium py-2 rounded-lg transition-colors"
                  >
                    Learn More About Eigenlayer
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
      
      <footer className="bg-midnight-900 border-t border-midnight-700 py-6 mt-12">
        <div className="container mx-auto px-4">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <div className="mb-4 md:mb-0">
              <div className="text-spirulina-500 font-bold text-xl">NOURISH CHAIN DAO</div>
              <div className="text-gray-400 text-sm mt-1">#SaveHumanity #NourishLife</div>
            </div>
            <div className="text-gray-500 text-sm">
              © 2025 Nourish Chain DAO. Created by Robert Patrick Campbell (Skhi Bridges)
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}
