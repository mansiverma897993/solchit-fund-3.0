import "./globals.css";
import Providers from "./providers";
import Navbar from "./components/Navbar";

export const metadata = {
  title: "SolChit Fund",
  description: "Decentralized Chit Fund on Solana",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-gradient-to-br from-[#f8fafc] via-[#eef2ff] to-[#ecfdf5] min-h-screen">
        <Providers>
          <Navbar />
          <main className="max-w-6xl mx-auto p-6">{children}</main>
        </Providers>
      </body>
    </html>
  );
}
