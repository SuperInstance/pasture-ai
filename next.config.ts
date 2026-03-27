import type { NextConfig } from "next";

const backendUrl = process.env.BACKEND_URL ?? "http://localhost:3001";

const nextConfig: NextConfig = {
  output: "standalone",
  reactStrictMode: true,

  // Frontend proxy to backend with WebSocket support
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: `${backendUrl}/api/:path*`,
      },
    ];
  },
  
  // WebSocket proxy configuration for development
  // Note: In production, use a reverse proxy (Caddy/Nginx) for WS
  experimental: {
    serverActions: {
      bodySizeLimit: '2mb',
    },
  },
};

export default nextConfig;
