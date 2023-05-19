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
          Free and open source mock API server for quick and easy mocking of
          RESTful APIs, with zero coding required.
          <br />
          <br />
          Get, create, update, and delete mock data using a simple and easy to
          use REST API. Its built with Rust, and is blazing fast.
        </p>

        <div className="options">
          <Link href="/download">
            <FaDownload className="icon" />
            <p className="text">Download</p>
          </Link>
          <Link href="/docs">
            <IoDocumentTextOutline className="icon" />
            <p className="text">Documentation</p>
          </Link>
        </div>
      </div>
    </div>
  );
}
