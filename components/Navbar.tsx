"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function Navbar() {
  const path = usePathname();

  const links = [
    {
      name: "Home",
      link: "/",
    },
    {
      name: "Docs",
      link: "/docs",
    },
    {
      name: "Visualize Data",
      link: "/visualize",
    },
    {
      name: "Github",
      link: "https://github.com/KRShanto/KRApi",
    },
    {
      name: "Download",
      link: "/download", // TODO: Add download page
      className: "download",
    },
  ];

  return (
    <nav id="navbar">
      <Link href="/" className="logo">
        <h1>KR Api</h1>
      </Link>

      <div className="links">
        {links.map((link, index) => (
          <Link
            href={link.link}
            key={index}
            className={`${path === link.link ? "active" : ""} ${
              link.className ?? ""
            }`}
          >
            {link.name}
          </Link>
        ))}
      </div>
    </nav>
  );
}
