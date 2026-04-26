import { Routes, Route } from 'react-router-dom';
import { SplashScreen } from '@/routes/SplashScreen';
import { Titlebar } from '@/components/ui/titlebar';

function App() {
    return (
        <>
            <Titlebar />
            <main className="mt-8">
                <Routes>
                    <Route path="/" element={<SplashScreen/>} />
                </Routes>
            </main>
        </>
    );
}

export default App;
