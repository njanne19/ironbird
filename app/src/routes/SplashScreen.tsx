import { ActionCard } from "@/components/ui/action-card";
import { HardDriveDownload, FolderInput, File } from "lucide-react";
import { Search } from "lucide-react";

import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/ui/input-group";

export function SplashScreen() {
    return (
        <div className="flex h-screen flex-col items-center justify-center">
            <div className="grid grid-cols-7 gap-4">
                <h1 className="font-pirata text-8xl col-span-7 text-left">Ironbird</h1>
                <div className="col-span-3">
                    <h1 className="text-3xl mb-4">Open recent</h1>
                    <InputGroup className="mb-4">
                        <InputGroupInput placeholder="Search..." />
                        <InputGroupAddon>
                            <Search/>
                        </InputGroupAddon>
                        <InputGroupAddon align="inline-end">12 results</InputGroupAddon>
                    </InputGroup> 
                </div>
                <div></div>
                <div className="col-span-3">
                    <h2 className="text-3xl mb-4">Get started</h2>
                    <ActionCard
                        icon={<HardDriveDownload size={36} />}
                        title="Clone a repository"
                        subtitle="Get a project from an online repository like GitHub"
                        className="mb-4"
                    />
                    <ActionCard
                        icon={<FolderInput size={36} />}
                        title="Open a project"
                        subtitle="Open a local Ironbird project or .ibd file"
                        className="mb-4"
                    />
                    <ActionCard
                        icon={<File size={36} />}
                        title="Create a new project"
                        subtitle="Start fresh!"
                        className="mb-4"
                    />
                </div>
            </div>
        </div>
    );
}
