import Footer from "@/components/Footer";
import Navbar from "@/components/Navbar";
import "@/styles/main.scss";

export const metadata = {
  title: "KR Api - Local api testing server",
  description: "A local api server for frontend applicaitons.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Navbar />
        <main>{children}</main>
        <Footer />
      </body>
    </html>
  );
}
