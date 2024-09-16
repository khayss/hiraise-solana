import {
  CreateCampaignIcon,
  DonateIcon,
  EnterIcon,
} from "@/components/icons/common";
import { HiRaise } from "@/components/icons/logo";
import { Button } from "@/components/ui/button";
import Link from "next/link";

export default function Home() {
  return (
    <div className="w-full px-8 md:px-12 lg:px-16 xl:px-32">
      <header>
        <nav className="flex justify-between items-center">
          <div className="flex justify-center items-center gap-2">
            <HiRaise width="32px" />
            <p className="text-xl font-semibold hidden md:block">HiRaise</p>
          </div>
          <ul>
            <li>
              <Link href={"/app"} target="_blank">
                <Button className="flex justify-center items-center gap-2">
                  <span>Open App</span>
                  <span>
                    <EnterIcon width="24px" />
                  </span>
                </Button>
              </Link>
            </li>
          </ul>
        </nav>
      </header>
      <div className="my-12 flex flex-col items-center gap-8">
        <div className="flex flex-col gap-2">
          <h1 className="text-center">HiRaise</h1>
          <p className="text-4xl font-bold text-center">
            A platform to raise funds for your projects and ideas.
          </p>
        </div>
        <div className="w-full flex flex-col md:flex-row md:justify-center gap-4">
          <Button className="w-full md:max-w-56 flex justify-center items-center gap-2">
            <span>Create a Campaign</span>
            <span>
              <CreateCampaignIcon width="16px" />
            </span>
          </Button>
          <Button className="w-full md:max-w-56 flex justify-center items-center gap-2">
            <span>Donate</span>
            <span>
              <DonateIcon width="16px" />
            </span>
          </Button>
        </div>
      </div>
    </div>
  );
}
