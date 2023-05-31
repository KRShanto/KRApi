import { useState, useEffect } from "react";
import Link from "next/link";
import { HiChevronDown, HiChevronUp } from "react-icons/hi";
import { ChapterType } from "@/types/chapter";
import Page from "./Page";

export default function Chapter({
  opens,
  active,
  chapter,
  index,
  setOpens,
}: {
  opens: number[];
  active: string | null;
  chapter: ChapterType;
  index: number;
  setOpens: any;
}) {
  // Is the chapter is open
  const [isOpen, setIsOpen] = useState<boolean>(false);
  // Is the chapter is active
  const isActive = active === "/docs" + chapter.href;
  // Does the chapter has any page
  const hasPages = chapter.pages && chapter.pages.length > 0;

  useEffect(() => {
    setIsOpen(opens.includes(index));
  }, [opens]);

  // check if any child pages are active
  // if so than make this (parent) open
  useEffect(() => {
    if (chapter.pages) {
      for (let i = 0; i < chapter.pages.length; i++) {
        if (active === "/docs" + chapter.pages[i].href) {
          // the page is active
          setIsOpen(true);
          break;
        }
      }
    }
  }, [active]);

  function toggleOpen() {
    if (hasPages) {
      // toggle the opens state
      if (isOpen) {
        setOpens(opens.filter((i) => i !== index));
      } else {
        setOpens([...opens, index]);
      }
    }
  }

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
          <button className="toggle" onClick={toggleOpen}>
            {isOpen ? <HiChevronUp /> : <HiChevronDown />}
          </button>
        )}
      </div>
      {isOpen && hasPages && (
        <div className="pages">
          {chapter.pages?.map((page: any, index: number) => (
            <Page page={page} active={active} key={index} />
          ))}
        </div>
      )}
    </div>
  );
}
