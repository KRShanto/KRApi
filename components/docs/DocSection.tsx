import React from "react";

export default function DocSection({
  children,
}: {
  children: React.ReactNode;
}) {
  return <div className="doc-section">{children}</div>;
}
