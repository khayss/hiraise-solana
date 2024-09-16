import { AppNavMenu } from "@/components/common/appNavMenu";

export default function AppLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="w-full">
      <div className="w-full">
        <AppNavMenu />
      </div>
      <div className="w-full">{children}</div>
    </div>
  );
}
