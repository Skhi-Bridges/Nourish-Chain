import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import Link from 'next/link';
import { Chart as ChartJS, ArcElement, Tooltip, Legend, CategoryScale, LinearScale, PointElement, LineElement, Title } from 'chart.js';
import { Doughnut, Line } from 'react-chartjs-2';

// Register ChartJS components
ChartJS.register(ArcElement, Tooltip, Legend, CategoryScale, LinearScale, PointElement, LineElement, Title);

// Mock data - in a real app this would come from your API
const mockSensorData = {
  ph: 7.2,
  temperature: 28.5,
  lightIntensity: 85,
  nutrientLevels: 92,
  harvestReadiness: 87,
  dailyGrowth: [42, 45, 48, 52, 56, 58, 62, 65, 68, 70, 73, 76, 78, 82],
  waterQuality: 94,
  carbonFootprint: 12, // CO2 equivalent (lower is better)
};

// Mock historical data for growth chart
const growthLabels = [...Array(14)].map((_, i) => `Day ${i + 1}`);

export default function Home() {
  const [sensorData, setSensorData] = useState(mockSensorData);
  
  // Simulate real-time data updates
  useEffect(() => {
    const interval = setInterval(() => {
      setSensorData(prev => ({
        ...prev,
        ph: +(prev.ph + (Math.random() * 0.2 - 0.1)).toFixed(1),
        temperature: +(prev.temperature + (Math.random() * 0.4 - 0.2)).toFixed(1),
        lightIntensity: Math.min(100, Math.max(0, prev.lightIntensity + Math.floor(Math.random() * 5 - 2))),
        waterQuality: Math.min(100, Math.max(0, prev.waterQuality + Math.floor(Math.random() * 3 - 1))),
      }));
    }, 5000);
    
    return () => clearInterval(interval);
  }, []);

  // Chart data
  const phData = {
    labels: ['Below Optimal', 'Optimal pH', 'Above Optimal'],
    datasets: [
      {
        data: [
          Math.max(0, 7.0 - sensorData.ph), 
          Math.min(1.0, Math.max(0, 1 - Math.abs(sensorData.ph - 7.0))), 
          Math.max(0, sensorData.ph - 7.0)
        ],
        backgroundColor: ['#f59e0b', '#22c55e', '#f59e0b'],
        borderColor: ['#d97706', '#16a34a', '#d97706'],
        borderWidth: 1,
      },
    ],
  };
  
  const growthData = {
    labels: growthLabels,
    datasets: [
      {
        label: 'Spirulina Growth Rate',
        data: sensorData.dailyGrowth,
        borderColor: '#22c55e',
        backgroundColor: 'rgba(34, 197, 94, 0.2)',
        borderWidth: 2,
        tension: 0.3,
        fill: true,
      },
    ],
  };

  // Utility to create gauge charts
  const createGaugeOption = (value, title, color = '#22c55e', dangerThreshold = 30, warningThreshold = 70) => {
    const gaugeColor = value < dangerThreshold ? '#ef4444' : value < warningThreshold ? '#f59e0b' : color;
    
    return {
      value,
      title,
      color: gaugeColor,
    };
  };

  // Gauge data
  const gauges = [
    createGaugeOption(sensorData.lightIntensity, 'Light %'),
    createGaugeOption(sensorData.nutrientLevels, 'Nutrients %'),
    createGaugeOption(sensorData.harvestReadiness, 'Harvest Ready %'),
    createGaugeOption(sensorData.waterQuality, 'Water Quality %'),
    createGaugeOption(100 - sensorData.carbonFootprint, 'Eco Score %'),
  ];

  return (
    <div className="min-h-screen bg-gradient-to-br from-midnight-900 via-midnight-800 to-midnight-900">
      <Head>
        <title>Nourish Chain - Spirulina Monitoring</title>
        <meta name="description" content="Nourish Chain DAO Spirulina Monitoring Dashboard" />
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
              <Link href="/" className="text-white font-medium hover:text-spirulina-400 border-b-2 border-spirulina-500">
                Spirulina Metrics
              </Link>
              <Link href="/staking" className="text-gray-300 font-medium hover:text-spirulina-400">
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
          {/* Left column - Spirulina Cultivation Status */}
          <div className="lg:col-span-2 space-y-6">
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-2xl font-bold text-spirulina-500 mb-4">Spirulina Cultivation Status</h2>
              
              <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                {/* Temperature */}
                <div className="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                  <div className="text-gray-400 text-sm mb-1">Temperature</div>
                  <div className="flex items-end">
                    <span className="text-2xl font-bold text-white">{sensorData.temperature}°C</span>
                    <span className="text-spirulina-500 text-sm ml-1">Optimal</span>
                  </div>
                </div>
                
                {/* pH Level with small chart */}
                <div className="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700">
                  <div className="text-gray-400 text-sm mb-1">pH Level</div>
                  <div className="flex items-end">
                    <span className="text-2xl font-bold text-white">{sensorData.ph}</span>
                    <span className="text-spirulina-500 text-sm ml-1">Balanced</span>
                  </div>
                </div>
                
                {/* Small pH gauge */}
                <div className="bg-midnight-900/50 rounded-lg p-4 border border-midnight-700 flex items-center justify-center">
                  <div className="w-24 h-24">
                    <Doughnut 
                      data={phData} 
                      options={{
                        cutout: '70%',
                        plugins: {
                          legend: { display: false },
                          tooltip: { enabled: false }
                        }
                      }} 
                    />
                    <div className="relative w-full h-0" style={{top: '-72px', textAlign: 'center'}}>
                      <div className="text-xs text-gray-400">pH</div>
                      <div className="text-xl font-bold text-white">{sensorData.ph}</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            {/* Growth chart */}
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Growth Trend (14 Days)</h2>
              <div className="h-80">
                <Line 
                  data={growthData} 
                  options={{
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                      y: {
                        beginAtZero: true,
                        max: 100,
                        grid: {
                          color: 'rgba(255, 255, 255, 0.1)'
                        },
                        ticks: {
                          color: 'rgba(255, 255, 255, 0.7)'
                        }
                      },
                      x: {
                        grid: {
                          color: 'rgba(255, 255, 255, 0.1)'
                        },
                        ticks: {
                          color: 'rgba(255, 255, 255, 0.7)'
                        }
                      }
                    },
                    plugins: {
                      legend: {
                        labels: {
                          color: 'rgba(255, 255, 255, 0.7)'
                        }
                      }
                    }
                  }} 
                />
              </div>
            </div>
          </div>
          
          {/* Right column - Metrics Gauges & Blockchain Info */}
          <div className="space-y-6">
            {/* Gauges */}
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Key Metrics</h2>
              
              <div className="space-y-4">
                {gauges.map((gauge, index) => (
                  <div key={index} className="flex items-center">
                    <div className="w-32 text-sm text-gray-400">{gauge.title}</div>
                    <div className="flex-1">
                      <div className="w-full bg-midnight-900 rounded-full h-4">
                        <div 
                          className="h-4 rounded-full transition-all duration-500 ease-out"
                          style={{ 
                            width: `${gauge.value}%`, 
                            backgroundColor: gauge.color 
                          }}
                        ></div>
                      </div>
                    </div>
                    <div className="w-12 text-right text-sm font-medium text-white">{gauge.value}%</div>
                  </div>
                ))}
              </div>
            </div>
            
            {/* Blockchain Info */}
            <div className="bg-midnight-800 rounded-xl p-6 border border-gold-700/30 shadow-xl">
              <h2 className="text-xl font-bold text-spirulina-500 mb-4">Blockchain Status</h2>
              
              <div className="space-y-3">
                <div className="flex justify-between">
                  <span className="text-gray-400">NRSH Price</span>
                  <span className="text-white font-medium">$1.28 <span className="text-green-500 text-xs">+3.2%</span></span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Your NRSH Balance</span>
                  <span className="text-white font-medium">42.6</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Staking Rewards</span>
                  <span className="text-white font-medium">0.62 NRSH / day</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Production Certified</span>
                  <span className="text-white font-medium">275 gal</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Last Block</span>
                  <span className="text-white font-medium">#4,328,651 (12s ago)</span>
                </div>
                
                <div className="mt-4 pt-4 border-t border-midnight-700">
                  <Link href="/staking" className="block w-full text-center bg-gold-600 hover:bg-gold-700 text-white font-medium py-2 rounded-lg transition-colors">
                    Manage Staking
                  </Link>
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
