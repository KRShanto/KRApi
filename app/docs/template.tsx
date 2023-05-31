import DocNav from "@/components/docs/DocNav";
import DocSection from "@/components/docs/DocSection";

export default function DocLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="docs">
      <DocNav />
      <DocSection>{children}</DocSection>
    </div>
  );
}
