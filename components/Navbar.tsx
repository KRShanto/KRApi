"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function Navbar() {
  const path = usePathname();
  const [activeLinks, setActiveLinks] = useState<string[]>([]);

  useEffect(() => {
    // Split the path by "/"
    const pathParts = path.split("/");
    console.log("Path: ", path);
    console.log("PathParts: ", pathParts);
    // If the path is not "/" then remove "/" from the array
    if (path !== "/") pathParts.shift();
    // Set the active links
    setActiveLinks(pathParts);
  }, [path]);

  console.log("ActiveLinks: ", activeLinks);

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
      name: "Github",
      link: "https://github.com/KRShanto/KRApi",
      external: true,
    },
    {
      name: "Download",
      link: "/download", // TODO: Add download page
      className: "download",
    },
  ];

  console.log("Path: ", path);

  return (
    <nav id="navbar">
      <Link href="/" className="logo">
        <h1>KR Api</h1>
      </Link>

      <div className="links">
        {links.map((link, index) => {
          // Check if the link is active
          // External links are not active
          const isActive =
            !link.external && activeLinks.includes(link.link.split("/")[1]);

          return (
            <Link
              href={link.link}
              key={index}
              className={`${isActive ? "active" : ""} ${link.className ?? ""}`}
              target={link.external ? "_blank" : ""}
            >
              {link.name}
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
