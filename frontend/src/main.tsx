import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import { ThemeProvider } from "@emotion/react";
import { createTheme } from "@mui/material";

const theme = createTheme({
    palette: {
        primary: {
            main: "#223884",
        },
        secondary: {
            main: "#FAB915",
        },
    },
});

ReactDOM.createRoot(document.getElementById("root")!).render(
    <ThemeProvider theme={theme}>
        <App />
    </ThemeProvider>
);