"use client";

export default function Home() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">

      {/* Stats Card */}
      <div className="bg-white rounded-2xl shadow-lg p-6">
        <h2 className="text-xl font-semibold text-gray-700">
          Total Pool Value
        </h2>
        <p className="text-3xl font-bold text-[#14F195] mt-4">
          0 USDC
        </p>
      </div>

      <div className="bg-white rounded-2xl shadow-lg p-6">
        <h2 className="text-xl font-semibold text-gray-700">
          Current Round
        </h2>
        <p className="text-3xl font-bold text-[#9945FF] mt-4">
          1
        </p>
      </div>

      {/* Members Table */}
      <div className="col-span-2 bg-white rounded-2xl shadow-lg p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-700">
          Members
        </h2>

        <table className="w-full text-left">
          <thead>
            <tr className="border-b">
              <th className="py-2">Wallet</th>
              <th>Deposited</th>
              <th>Won</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b">
              <td className="py-2">---</td>
              <td>No</td>
              <td>No</td>
            </tr>
          </tbody>
        </table>
      </div>

      {/* Action Buttons */}
      <div className="col-span-2 flex gap-4 mt-4">
        <button className="bg-[#14F195] hover:bg-green-400 text-black px-6 py-3 rounded-xl font-semibold transition w-full">
          Create Pool
        </button>

        <button className="bg-[#9945FF] hover:bg-purple-700 text-white px-6 py-3 rounded-xl font-semibold transition w-full">
          Deposit
        </button>
      </div>

    </div>
  );
}
