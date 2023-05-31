"use client";

import { useState, useEffect } from "react";
import { usePathname } from "next/navigation";
import { chapters } from "@/constants/chapters";
import Chapter from "@/components/docs/Chapter";

export default function DocNav() {
  const [opens, setOpens] = useState<number[]>([]);
  const [active, setActive] = useState<string | null>(null);
  const path = usePathname();

  useEffect(() => {
    setActive(path);
  }, [path]);

  return (
    <div className="doc-nav">
      {chapters.map((chapter, index) => (
        <Chapter
          opens={opens}
          active={active}
          chapter={chapter}
          index={index}
          key={index}
          setOpens={setOpens}
        />
      ))}
    </div>
  );
}
