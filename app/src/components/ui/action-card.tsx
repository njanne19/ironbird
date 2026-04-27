import * as React from "react";
import { cn } from "@/lib/utils"
import { cva, type VariantProps } from "class-variance-authority";

const actionCardContainer = cva(
    "grid grid-cols-6 text-wrap select-none transition-all",
    {
        variants: {
            variant: {
                default: "bg-muted hover:bg-muted/70 border-l-4 border-primary",
                ghost: "hover:bg-muted",
                outline: "border border-border hover:bg-muted",
            },
            size: {
                sm: "p-2 gap-1",
                md: "p-3 gap-2",
                lg: "p-5 gap-3",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "md",
        },
    }
)

const actionCardTitle = cva("font-semibold", {
    variants: {
        size: {
            sm: "text-base",
            md: "text-xl",
            lg: "text-2xl",
        },
    },
    defaultVariants: { size: "md" },
})

const actionCardSubtitle = cva("", {
    variants: {
        size: {
            sm: "text-xs",
            md: "text-sm",
            lg: "text-base",
        },
    },
    defaultVariants: { size: "md" },
})

interface ActionCardProps extends VariantProps<typeof actionCardContainer> {
    title: string;
    icon?: React.ReactNode;
    subtitle?: string;
    onClick?: () => void;
    className?: string;
}

function ActionCard({ title, icon, subtitle, onClick, className, variant, size }: ActionCardProps) {
    return (
        <div className={cn(actionCardContainer({ variant, size }), className)} onClick={onClick}>
            {icon &&
                <>
                    <div className="col-span-1 flex flex-col justify-center items-center">
                        <div>{icon}</div>
                    </div>
                    <div className="col-span-5">
                        <h1 className={actionCardTitle({ size })}>{title}</h1>
                        {subtitle && <p className={actionCardSubtitle({ size })}>{subtitle}</p>}
                    </div>
                </>
            }
            {!icon &&
                <>
                    <div className="col-span-6">
                        <h1 className={actionCardTitle({ size })}>{title}</h1>
                        {subtitle && <p className={actionCardSubtitle({ size })}>{subtitle}</p>}
                    </div>
                </>
            }
        </div>
    );
}

export { ActionCard, type ActionCardProps };
