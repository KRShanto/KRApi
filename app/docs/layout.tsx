import DocSection from "@/components/docs/DocSection";

export default function DocLayout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <DocSection>{children}</DocSection>
    </>
  );
}
