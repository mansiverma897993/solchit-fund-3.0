"use client";

import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export default function Navbar() {
  return (
    <div className="w-full bg-white shadow-md px-8 py-4 flex justify-between items-center">
      <h1 className="text-2xl font-bold text-[#7c3aed]">
        SolChit Dashboard
      </h1>

      <WalletMultiButton />
    </div>
  );
}
