import Link from "next/link";
import React from "react";
// `install` icon from "react-icons"
import { FaDownload } from "react-icons/fa";
import { IoDocumentTextOutline } from "react-icons/io5";
import { RiEyeLine } from "react-icons/ri";

export default function Home() {
  return (
    <div id="home">
      <div className="hero">
        <h1 className="heading">
          A <span className="highlight">Rust</span> based{" "}
          <span className="highlight">Web Server</span> for Mock APIs
        </h1>

        <p className="description">
          Empower your frontend development with KR Api, the game-changing
          solution for generating mock APIs at your command. With our Rust Web
          Server, you can effortlessly simulate and manipulate your testing
          environment. Take control of your projects, iterate with confidence,
          and unlock new levels of efficiency. Revolutionize your frontend
          development journey with the power of KR Api.
        </p>

        <div className="options">
          <Link href="/download">
            <FaDownload className="icon" />
            <p className="text">Download</p>
          </Link>
          <Link href="/visualize">
            <RiEyeLine className="icon" />
            <p className="text">Visualize your Data</p>
          </Link>
          <Link href="/docs">
            <IoDocumentTextOutline className="icon" />
            <p className="text">Docs</p>
          </Link>
        </div>
      </div>
    </div>
  );
}
