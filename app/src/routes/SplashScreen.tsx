import { ActionCard } from "@/components/ui/action-card";
import { HardDriveDownload, FolderInput, File, Shapes } from "lucide-react";
import { Search } from "lucide-react";
import { useEffect, useState } from "react";
import { useIronbirdAppStore } from "@/store/app";
import { commands } from "@/bindings";
import { useNavigate } from "react-router-dom";

import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/ui/input-group";

import { 
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";


export function SplashScreen() {
    const setTitlebarLabel = useIronbirdAppStore((state) => state.setTitlebarLabel);
    useEffect(() => {
        setTitlebarLabel("Ironbird - Welcome");
    })

    const [demoPickerOpen, setDemoPickerOpen] = useState(false);

    const navigate = useNavigate();
    const setActiveProject = useIronbirdAppStore((state) => state.setActiveProject);

    async function handleDemoSelect(demoName: string) {
        const result = await commands.loadDemoFileAsIbd(demoName)
        if (result.status === 'ok') {
            console.log("Got a demo object back: ", result.data);
            setActiveProject(result.data)
            navigate('/project')
        } else {
            console.error("Failed to load demo: ", result.error);
        }
    }

    return (
        <div className="flex h-screen flex-col items-center justify-center px-10">
            <div className="grid grid-cols-7 gap-4 w-full max-w-5xl">
                <h1 className="font-pirata text-8xl col-span-7 text-left select-none">Ironbird</h1>
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
                        icon={<Shapes size={36} />}
                        title="Try out a demo"
                        subtitle="Pick from one of our example scenarios to get started. Edit as much as you want and save as your own later."
                        className="mb-4"
                        onClick={() => setDemoPickerOpen(true)}
                    />
                    {/* Diaglog for the demo picker. Actually gets teleported to the right area on open */}
                    <Dialog open={demoPickerOpen} onOpenChange={() => setDemoPickerOpen(false)}>
                       <DialogContent>
                            <DialogHeader className="mb-4">
                                <DialogTitle>Choose a demo</DialogTitle>
                            </DialogHeader>
                            <ActionCard
                                title="Simple pendulum"
                                subtitle="A simple 1D pendulum example showing off simulation and modelling basics"
                                size="sm"
                                onClick={() => handleDemoSelect("simple-pendulum")}
                            />
                       </DialogContent>
                    </Dialog>

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
