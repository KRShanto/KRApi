import { ChapterType } from "@/types/chapter";
import Link from "next/link";

export default function Page({
  active,
  page,
}: {
  active: string | null;
  page: ChapterType;
}) {
  // Is the page is active
  const isActive = active === "/docs" + page.href;

  return (
    <Link
      className={`page ${isActive ? "active" : ""}`}
      href={`docs/${page.href}`}
    >
      {page.title}
    </Link>
  );
}
