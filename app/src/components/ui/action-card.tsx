import * as React from "react";
import { cn } from "@/lib/utils"

interface ActionCardProps {
    title: string;
    icon?: React.ReactNode;
    subtitle?: string;
    onClick?: () => void;
    className?: string;
}

function ActionCard(props: ActionCardProps) {
    return (
        <div className={cn("grid grid-cols-6 cursor-default bg-muted hover:bg-muted/70 transition-all p-3", props.className)} onClick={props.onClick}> 
            <div className="col-span-1 flex flex-col justify-center items-center">
                {props.icon && <div>{props.icon}</div>} 
            </div> 
            <div className="col-span-5">
                <h1 className="text-xl font-semibold">{props.title}</h1>
                {props.subtitle && <p className="text-sm">{props.subtitle}</p>}
            </div> 
        </div>
    );
}

export { ActionCard, type ActionCardProps };
