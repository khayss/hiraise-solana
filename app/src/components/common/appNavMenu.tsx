import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import Link from "next/link";
import { WalletConnectButton } from "../wallet/walletConnectButton";

const components: {
  id: number;
  name: string;
  link: string;
}[] = [
  {
    id: 1,
    name: "dashboard",
    link: "/app",
  },
  {
    id: 2,
    name: "create campaign",
    link: "/app/create",
  },
  {
    id: 3,
    name: "current campaigns",
    link: "/app/campaigns",
  },
  {
    id: 4,
    name: "past campaigns",
    link: "/app/past-campaigns",
  },
];

export function AppNavMenu() {
  return (
    <div className="py-2 px-4 bg-muted rounded-lg w-full flex flex-col gap-4">
      <NavigationMenu>
        <NavigationMenuList className="flex flex-col gap-2 items-start">
          {components.map((component) => (
            <NavigationMenuItem key={component.id}>
              <Link
                href={component.link}
                className={navigationMenuTriggerStyle()}
              >
                {component.name}
              </Link>
            </NavigationMenuItem>
          ))}
        </NavigationMenuList>
      </NavigationMenu>
      <WalletConnectButton />
    </div>
  );
}
