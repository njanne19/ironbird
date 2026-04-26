export function SplashScreen() {
    return (
        <div className="flex h-screen flex-col items-center justify-center">
            <div className = "grid grid-cols-7 gap-4">
                <h1 className="font-pirata text-7xl col-span-7 text-left">Ironbird</h1>
                <div className="col-span-3">
                    <h1>Welcome to Ironbird!</h1>
                </div>
                <div></div>
                <div className="col-span-3">
                    <h2>How would you like to begin?</h2>
                </div>
            </div>
        </div>
    );
}
