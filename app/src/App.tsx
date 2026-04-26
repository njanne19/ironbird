import { Routes, Route } from 'react-router-dom';
import { SplashScreen } from './routes/SplashScreen';

function App() {
    return (
        <Routes>
            <Route path="/" element={<SplashScreen/>} />
        </Routes>
    );
}

export default App;
