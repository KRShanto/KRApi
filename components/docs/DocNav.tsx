"use client";

import { useState, useEffect } from "react";
import { usePathname } from "next/navigation";
import Link from "next/link";
import { HiChevronDown, HiChevronUp } from "react-icons/hi";
import { chapters } from "@/constants/chapters";

export default function DocNav() {
  const [opens, setOpens] = useState<number[]>([]);
  const [active, setActive] = useState<string | null>(null);
  const path = usePathname();

  useEffect(() => {
    setActive(path);
  }, [path]);

  return (
    <div className="doc-nav">
      {chapters.map((chapter, index) => {
        // Is the chapter is open
        let isOpen = opens.includes(index);
        // Is the chapter is active
        let isActive = active === "/docs" + chapter.href;
        // check if any child pages are active
        // if so than make this (parent) open
        if (chapter.pages) {
          for (let i = 0; i < chapter.pages.length; i++) {
            if (active === "/docs" + chapter.pages[i].href) {
              // the page is active
              isOpen = true;
              break;
            }
          }
        }
        // Does the chapter has any page
        const hasPages = chapter.pages && chapter.pages.length > 0;

        return (
          <div key={index} className="chapter">
            <div className={`header ${isOpen ? "open" : ""}`}>
              <Link
                href={`docs/${chapter.href}`}
                className={`title ${isActive ? "active" : ""}`}
              >
                {chapter.title}
              </Link>
              {hasPages && (
                <button
                  className="toggle"
                  onClick={() => {
                    if (hasPages) {
                      // toggle the opens state
                      if (isOpen) {
                        setOpens(opens.filter((i) => i !== index));
                      } else {
                        setOpens([...opens, index]);
                      }
                    }
                  }}
                >
                  {isOpen ? <HiChevronUp /> : <HiChevronDown />}
                </button>
              )}
            </div>
            {(isOpen || isActive) && hasPages && (
              <div className="pages">
                {chapter.pages.map((page, index) => {
                  // Is the page is active
                  const isActive = active === "/docs" + page.href;

                  return (
                    <Link
                      className={`page ${isActive ? "active" : ""}`}
                      href={`docs/${page.href}`}
                      key={index}
                    >
                      {page.title}
                    </Link>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
