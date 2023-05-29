import DocNav from "@/components/docs/DocNav";

export default function DocLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <DocNav />
      {children}
    </>
  );
}
